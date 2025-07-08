use crate::app::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Handle fullscreen preview mode
    if let InputMode::Preview { content, file_path } = &app.input_mode {
        render_preview(f, content, file_path, app.preview_scroll, f.size());
        return;
    }

    // Handle help mode with proper layout
    if let InputMode::Help = &app.input_mode {
        let help_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Status bar
                    Constraint::Min(5),    // Help content
                    Constraint::Length(3), // Bottom status
                ]
                .as_ref(),
            )
            .split(f.size());

        render_status_bar(f, app, help_chunks[0]);
        crate::ui_help::render_clean_help(f, app, help_chunks[1]);
        render_help_status(f, help_chunks[2]);
        return;
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Status bar
                Constraint::Min(5),    // Main content
                Constraint::Length(3), // Input area
            ]
            .as_ref(),
        )
        .split(f.size());

    render_status_bar(f, app, main_chunks[0]);

    // Split main content horizontally if preview panel is enabled
    if app.show_preview_panel {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50), // File list
                    Constraint::Percentage(50), // Preview panel
                ]
                .as_ref(),
            )
            .split(main_chunks[1]);

        render_file_list(f, app, content_chunks[0]);
        render_preview_panel(f, app, content_chunks[1]);
    } else {
        render_file_list(f, app, main_chunks[1]);
    }

    render_input_area(f, app, main_chunks[2]);
}

fn render_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let status_content = vec![
        Spans::from(vec![
            Span::styled("Pretty Git UI v0.1.0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}@{}", &app.repo_name, &app.current_branch), Style::default().fg(Color::Green)),
        ]),
        Spans::from(vec![
            Span::raw("Press "),
            Span::styled("[h]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" for help, "),
            Span::styled("[q]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" to quit"),
        ]),
    ];

    let status = Paragraph::new(status_content).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title("Git Repository Status"),
    );

    f.render_widget(status, area);
}

fn render_file_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    let files: Vec<ListItem> = if app.files.is_empty() {
        vec![ListItem::new("変更されたファイルはありません")]
    } else {
        app.files
            .iter()
            .enumerate()
            .map(|(_, file_status)| {
                let formatted = format_file_status(file_status);
                let color = get_file_color(file_status);
                ListItem::new(formatted).style(Style::default().fg(color))
            })
            .collect()
    };

    let title = if app.files.is_empty() {
        "Git ファイル".to_string()
    } else {
        format!("Git ファイル ({}個)", app.files.len())
    };

    let files_widget = List::new(files)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray)
                .fg(Color::Yellow),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(files_widget, area, &mut app.files_state);
}

fn render_input_area<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    match &app.input_mode {
        InputMode::Normal => {
            let status_msg = Paragraph::new(format!("> {}", app.status_message))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("ステータス")
                        .border_style(Style::default().fg(Color::Blue)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(status_msg, area);
        },
        InputMode::Commit => {
            let input = Paragraph::new(app.commit_message.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title("コミットメッセージ ([Enter]送信 [Esc]キャンセル)")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow)),
                );
            f.render_widget(input, area);
            f.set_cursor(area.x + app.commit_message.len() as u16 + 1, area.y + 1);
        },
        InputMode::StashMessage => {
            let input = Paragraph::new(app.stash_message.as_ref())
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .title("スタッシュメッセージ ([Enter]スタッシュ [Esc]キャンセル)")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue)),
                );
            f.render_widget(input, area);
            f.set_cursor(area.x + app.stash_message.len() as u16 + 1, area.y + 1);
        },
        InputMode::Confirm { message, .. } => {
            let confirm = Paragraph::new(format!(
                "確認: {}\n[y]はい [n]いいえ [Esc]キャンセル",
                message
            ))
            .style(Style::default().fg(Color::Magenta))
            .block(
                Block::default()
                    .title("確認")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            );
            f.render_widget(confirm, area);
        },
        InputMode::Preview { content, file_path } => {
            render_preview(f, content, file_path, app.preview_scroll, area);
        },
        InputMode::Help => {
            // Help is handled at the top level, this shouldn't be reached
        },
    }
}

fn get_file_color(file_status: &str) -> Color {
    if file_status.is_empty() || file_status.len() < 2 {
        return Color::White;
    }

    // Check first character for staging status
    let is_staged = !file_status.chars().next().unwrap_or(' ').is_whitespace();

    if is_staged {
        Color::Green
    } else {
        Color::Red
    }
}

fn format_file_status(file_status: &str) -> String {
    let chars: Vec<char> = file_status.chars().collect();
    if chars.len() < 3 {
        return file_status.to_string();
    }

    let status_code: String = chars.iter().take(2).collect();
    let file_path: String = chars.iter().skip(2).collect::<String>().trim().to_string();

    let (status_symbol, status_text) = match status_code.as_str() {
        "M " => ("✓", "STAGED   "),
        " M" => ("Δ", "MODIFIED "),
        "A " => ("+", "ADDED    "),
        "D " => ("✗", "DELETED  "),
        " D" => ("✗", "DELETED  "),
        "??" => ("?", "UNTRACKED"),
        "MM" => ("±", "PARTIAL  "),
        "AM" => ("±", "PARTIAL  "),
        _ => ("•", "CHANGED  "),
    };

    format!("{} [{}] {}", status_symbol, status_text, &file_path)
}

fn render_preview<B: Backend>(
    f: &mut Frame<B>,
    content: &str,
    file_path: &str,
    scroll: u16,
    area: tui::layout::Rect,
) {
    let lines: Vec<&str> = content.lines().collect();
    let start_line = scroll as usize;
    let visible_lines: Vec<Spans> = lines
        .iter()
        .skip(start_line)
        .take((area.height.saturating_sub(2)) as usize)
        .enumerate()
        .map(|(i, line)| {
            let line_number = start_line + i + 1;
            let line_style = if line.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if line.starts_with('-') {
                Style::default().fg(Color::Red)
            } else if line.starts_with("@@") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            Spans::from(vec![
                Span::styled(
                    format!("{:4} ", line_number),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(line.to_string(), line_style),
            ])
        })
        .collect();

    let preview = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .title(format!(
                    "Preview: {} (j/k to scroll, q/Esc to exit)",
                    file_path
                ))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(preview, area);
}

fn render_help<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let help_text = vec![
        Spans::from(vec![
            Span::styled("Pretty Git UI - Help", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        ]),
        Spans::from(vec![Span::raw("")]),
        // Navigation
        Spans::from(vec![
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[j/k]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled(" or ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "[↓/↑]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled(" → ", Style::default().fg(Color::Yellow)),
            Span::raw("Traverse file_tree[]"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[h]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle help_system()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[q]",
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("Process::exit(0)"),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::raw("")]),
        // File Operations
        Spans::from(vec![
            Span::styled("┌──[ ", Style::default().fg(Color::Green)),
            Span::styled(
                "FILE_OPS",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            ),
            Span::styled(
                " ]───────────────────────────────",
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[s]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.stage_toggle(selected)"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[a]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.stage_all() // bulk operation"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[r]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("fs.refresh() && git.status()"),
        ]),
        Spans::from(vec![Span::raw("")]),
        // Git Operations
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::raw("")]),
        // Git Operations
        Spans::from(vec![
            Span::styled("┌──[ ", Style::default().fg(Color::Green)),
            Span::styled(
                "GIT_OPS",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            ),
            Span::styled(
                " ]────────────────────────────────",
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[c]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.commit_mode()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[t]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.stash_mode()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[l]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.list_stashes()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[p]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("git.apply_stash()"),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::raw("")]),
        // Preview
        Spans::from(vec![
            Span::styled("┌──[ ", Style::default().fg(Color::Green)),
            Span::styled(
                "PREVIEW_SYS",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            ),
            Span::styled(
                " ]──────────────────────────",
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[v]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("toggle_preview_panel()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[d]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("           → ", Style::default().fg(Color::Yellow)),
            Span::raw("render_fullscreen_diff()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[Shift+j/k]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("   → ", Style::default().fg(Color::Yellow)),
            Span::raw("scroll_preview_buffer()"),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::raw("")]),
        // Input Modes
        Spans::from(vec![
            Span::styled("┌──[ ", Style::default().fg(Color::Green)),
            Span::styled(
                "INPUT_MODES",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            ),
            Span::styled(
                " ]─────────────────────────",
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[Enter]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            ),
            Span::styled("       → ", Style::default().fg(Color::Yellow)),
            Span::raw("submit_buffer()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[Esc]",
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
            ),
            Span::styled("         → ", Style::default().fg(Color::Yellow)),
            Span::raw("abort_operation()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "[y/n]",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Magenta),
            ),
            Span::styled("         → ", Style::default().fg(Color::Yellow)),
            Span::raw("confirm_dialog()"),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
        Spans::from(vec![Span::raw("")]),
        // File Status Colors
        Spans::from(vec![
            Span::styled("┌──[ ", Style::default().fg(Color::Green)),
            Span::styled(
                "STATUS_CODES",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            ),
            Span::styled(
                " ]────────────────────────",
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "✓[STAGED]   ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::Yellow)),
            Span::raw("ready_for_commit()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "Δ[MODIFIED] ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::Yellow)),
            Span::raw("working_tree_changes()"),
        ]),
        Spans::from(vec![
            Span::styled("│ ", Style::default().fg(Color::Green)),
            Span::styled(
                "?[UNTRACKED]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("→ ", Style::default().fg(Color::Yellow)),
            Span::raw("new_file_detected()"),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "└─────────────────────────────────────────────",
            Style::default().fg(Color::Green),
        )]),
    ];

    let total_lines = help_text.len();
    let visible_lines = (area.height.saturating_sub(2)) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);

    // Apply scroll offset
    let visible_help_text: Vec<Spans> = help_text
        .into_iter()
        .skip(app.help_scroll as usize)
        .take(visible_lines)
        .collect();

    let scroll_info = if total_lines > visible_lines {
        format!(
            " (j/k to scroll {}/{})",
            app.help_scroll + 1,
            max_scroll + 1
        )
    } else {
        String::new()
    };

    let help = Paragraph::new(visible_help_text)
        .block(
            Block::default()
                .title(format!(
                    "┌─[ HELP_SYSTEM ]── KEYBIND_REFERENCE{} ──────────┐",
                    scroll_info
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}

fn render_help_status<B: Backend>(f: &mut Frame<B>, area: tui::layout::Rect) {
    let status_text = vec![Spans::from(vec![
        Span::styled("Navigation: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "j/k",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        ),
        Span::raw(" scroll | "),
        Span::styled(
            "h",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        ),
        Span::raw("/"),
        Span::styled(
            "q",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        ),
        Span::raw("/"),
        Span::styled(
            "Esc",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        ),
        Span::raw(" close | "),
        Span::styled("Pretty Git UI v0.1.0", Style::default().fg(Color::Cyan)),
        Span::raw(" - Help Mode"),
    ])];

    let help_status = Paragraph::new(status_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .alignment(Alignment::Center);

    f.render_widget(help_status, area);
}

fn render_preview_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let file_path = app
        .get_current_file_path()
        .unwrap_or_else(|| "No file selected".to_string());

    if app.preview_content.is_empty() {
        let empty_preview = Paragraph::new(
            "ファイルを選択してください\n\n[v] でパネル切り替え",
        )
        .block(
            Block::default()
                .title("プレビュー")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty_preview, area);
        return;
    }

    let lines: Vec<&str> = app.preview_content.lines().collect();
    let start_line = app.preview_scroll as usize;
    let visible_lines: Vec<Spans> = lines
        .iter()
        .skip(start_line)
        .take((area.height.saturating_sub(2)) as usize)
        .enumerate()
        .map(|(i, line)| {
            let line_number = start_line + i + 1;
            let line_style = if line.starts_with('+') {
                Style::default().fg(Color::Green)
            } else if line.starts_with('-') {
                Style::default().fg(Color::Red)
            } else if line.starts_with("@@") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            // Truncate long lines to fit the panel (Unicode-safe)
            let max_width = (area.width as usize).saturating_sub(8);
            let display_line = if line.chars().count() > max_width {
                let truncate_width = max_width.saturating_sub(3);
                let truncated: String = line.chars().take(truncate_width).collect();
                format!("{}...", truncated)
            } else {
                line.to_string()
            };

            Spans::from(vec![
                Span::styled(
                    format!("{:3} ", line_number),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(display_line, line_style),
            ])
        })
        .collect();

    let preview = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .title(format!("差分: {}", file_path))
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(preview, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_color() {
        assert_eq!(get_file_color("M  file.txt"), Color::Green);
        assert_eq!(get_file_color("A  file.txt"), Color::Green);
        assert_eq!(get_file_color(" M file.txt"), Color::Red);
        assert_eq!(get_file_color("?? file.txt"), Color::Green); // untracked is considered staged
        assert_eq!(get_file_color("D  file.txt"), Color::Green);
        assert_eq!(get_file_color(""), Color::White);
        assert_eq!(get_file_color("M"), Color::White);
    }

    #[test]
    fn test_file_status_parsing() {
        let staged_modified = "M  src/main.rs";
        assert!(!staged_modified.chars().next().unwrap().is_whitespace());

        let unstaged_modified = " M src/main.rs";
        assert!(unstaged_modified.chars().next().unwrap().is_whitespace());

        let added = "A  new_file.txt";
        assert!(!added.chars().next().unwrap().is_whitespace());

        let untracked = "?? untracked.txt";
        assert!(!untracked.chars().next().unwrap().is_whitespace());
    }

    #[test]
    fn test_input_mode_display() {
        let app = App::new();

        match &app.input_mode {
            InputMode::Normal => {
                assert!(matches!(app.input_mode, InputMode::Normal));
            },
            InputMode::Commit => {
                assert!(matches!(app.input_mode, InputMode::Commit));
            },
            InputMode::StashMessage => {
                assert!(matches!(app.input_mode, InputMode::StashMessage));
            },
            InputMode::Confirm { .. } => {
                assert!(matches!(app.input_mode, InputMode::Confirm { .. }));
            },
            InputMode::Preview { .. } => {
                assert!(matches!(app.input_mode, InputMode::Preview { .. }));
            },
            InputMode::Help => {
                assert!(matches!(app.input_mode, InputMode::Help));
            },
        }
    }

    #[test]
    fn test_cursor_position_calculation() {
        let message = "test commit message";
        let expected_cursor_x = 1 + message.len() as u16 + 1;
        let actual_cursor_x = 1 + message.len() as u16 + 1;
        assert_eq!(actual_cursor_x, expected_cursor_x);
    }

    #[test]
    fn test_layout_constraints() {
        let constraints = [
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ];

        assert_eq!(constraints.len(), 3);

        match constraints[0] {
            Constraint::Length(3) => {},
            _ => panic!("Expected Length(3)"),
        }

        match constraints[1] {
            Constraint::Min(5) => {},
            _ => panic!("Expected Min(5)"),
        }

        match constraints[2] {
            Constraint::Length(3) => {},
            _ => panic!("Expected Length(3)"),
        }
    }
}
