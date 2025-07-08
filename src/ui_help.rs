use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App;

pub fn render_clean_help<B: Backend>(f: &mut Frame<B>, app: &App, area: tui::layout::Rect) {
    let help_text = vec![
        Spans::from(vec![
            Span::styled("Pretty Git UI - ヘルプ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // Navigation
        Spans::from(vec![
            Span::styled("ナビゲーション:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  j/k ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("または "),
            Span::styled("↓/↑  ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("ファイル移動")
        ]),
        Spans::from(vec![
            Span::styled("  h     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("ヘルプ表示")
        ]),
        Spans::from(vec![
            Span::styled("  q     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
            Span::raw("アプリケーション終了")
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // File Operations
        Spans::from(vec![
            Span::styled("ファイル操作:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  s     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("選択ファイルをステージ/アンステージ")
        ]),
        Spans::from(vec![
            Span::styled("  a     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("すべてのファイルをステージ/アンステージ")
        ]),
        Spans::from(vec![
            Span::styled("  r     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("ファイルリスト更新")
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // Git Operations
        Spans::from(vec![
            Span::styled("Git操作:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  c     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("コミットメッセージ入力")
        ]),
        Spans::from(vec![
            Span::styled("  t     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("スタッシュメッセージ入力")
        ]),
        Spans::from(vec![
            Span::styled("  l     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("スタッシュ一覧表示")
        ]),
        Spans::from(vec![
            Span::styled("  p     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("最新スタッシュ適用")
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // Preview
        Spans::from(vec![
            Span::styled("プレビュー:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  v     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("プレビューパネル切り替え")
        ]),
        Spans::from(vec![
            Span::styled("  d     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("フルスクリーン差分表示")
        ]),
        Spans::from(vec![
            Span::styled("  Shift+j/k ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("プレビューパネルスクロール")
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // Input Modes
        Spans::from(vec![
            Span::styled("入力モード:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  Enter ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            Span::raw("送信 (コミット/スタッシュモード)")
        ]),
        Spans::from(vec![
            Span::styled("  Esc   ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
            Span::raw("キャンセル")
        ]),
        Spans::from(vec![
            Span::styled("  y/n   ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta)),
            Span::raw("確認/拒否 (確認モード)")
        ]),
        Spans::from(vec![Span::raw("")]),
        
        // File Status
        Spans::from(vec![
            Span::styled("ファイル状態:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("緑色", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  ステージ済み (コミット準備完了)")
        ]),
        Spans::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("赤色", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("  変更済み (未ステージ)")
        ]),
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
        format!(" (j/kでスクロール {}/{})", app.help_scroll + 1, max_scroll + 1)
    } else {
        String::new()
    };

    let help = Paragraph::new(visible_help_text)
        .block(
            Block::default()
                .title(format!("ヘルプ{}", scroll_info))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}