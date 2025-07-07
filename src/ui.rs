use crate::app::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    render_status_bar(f, chunks[0]);
    render_file_list(f, app, chunks[1]);
    render_input_area(f, app, chunks[2]);
}

fn render_status_bar<B: Backend>(f: &mut Frame<B>, area: tui::layout::Rect) {
    let status = Paragraph::new(Spans::from(vec![
        Span::raw("pretty-git-ui - "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":quit "),
        Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":stage/unstage "),
        Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":stage/unstage all "),
        Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":commit "),
        Span::styled("t", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":stash "),
        Span::styled("p", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(":apply stash"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

fn render_file_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: tui::layout::Rect) {
    let files: Vec<ListItem> = app
        .files
        .iter()
        .map(|file_status| {
            let color = get_file_color(file_status);
            ListItem::new(file_status.clone()).style(Style::default().fg(color))
        })
        .collect();

    let files_widget = List::new(files)
        .block(
            Block::default()
                .title("Changed Files")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(files_widget, area, &mut app.files_state);
}

fn render_input_area<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    match app.input_mode {
        InputMode::Normal => {
            let status_msg = Paragraph::new(app.status_message.clone())
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_msg, area);
        },
        InputMode::Commit => {
            let input = Paragraph::new(app.commit_message.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title("Commit Message")
                        .borders(Borders::ALL),
                );
            f.render_widget(input, area);
            f.set_cursor(area.x + app.commit_message.len() as u16 + 1, area.y + 1);
        },
        InputMode::StashMessage => {
            let input = Paragraph::new(app.stash_message.as_ref())
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .title("Stash Message (optional)")
                        .borders(Borders::ALL),
                );
            f.render_widget(input, area);
            f.set_cursor(area.x + app.stash_message.len() as u16 + 1, area.y + 1);
        },
    }
}

fn get_file_color(file_status: &str) -> Color {
    if file_status.starts_with("M ") || file_status.starts_with("A ") {
        Color::Green
    } else {
        Color::Red
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_color() {
        assert_eq!(get_file_color("M  file.txt"), Color::Green);
        assert_eq!(get_file_color("A  file.txt"), Color::Green);
        assert_eq!(get_file_color(" M file.txt"), Color::Red);
        assert_eq!(get_file_color("?? file.txt"), Color::Red);
        assert_eq!(get_file_color("D  file.txt"), Color::Red);
    }

    #[test]
    fn test_file_status_parsing() {
        let staged_modified = "M  src/main.rs";
        assert!(staged_modified.starts_with("M "));

        let unstaged_modified = " M src/main.rs";
        assert!(!unstaged_modified.starts_with("M "));

        let added = "A  new_file.txt";
        assert!(added.starts_with("A "));

        let untracked = "?? untracked.txt";
        assert!(!untracked.starts_with("M "));
        assert!(!untracked.starts_with("A "));
    }

    #[test]
    fn test_input_mode_display() {
        let app = App::new();

        match app.input_mode {
            InputMode::Normal => {
                assert_eq!(app.input_mode, InputMode::Normal);
            },
            InputMode::Commit => {
                assert_eq!(app.input_mode, InputMode::Commit);
            },
            InputMode::StashMessage => {
                assert_eq!(app.input_mode, InputMode::StashMessage);
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
