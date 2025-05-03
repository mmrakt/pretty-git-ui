const VERSION: &str = "0.1.0";

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    process::Command,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

enum InputMode {
    Normal,
    Commit,
}

struct App {
    files: Vec<String>,
    files_state: ListState,
    input_mode: InputMode,
    commit_message: String,
    status_message: String,
}

impl App {
    fn new() -> App {
        let mut app = App {
            files: Vec::new(),
            files_state: ListState::default(),
            input_mode: InputMode::Normal,
            commit_message: String::new(),
            status_message: String::from(format!("Git TUI v{} - Welcome", VERSION)),
        };
        app.refresh_files();
        if !app.files.is_empty() {
            app.files_state.select(Some(0));
        }
        app
    }

    /// リポジトリの変更状態を取得し、表示用に整形
    fn refresh_files(&mut self) {
        let output = match Command::new("git").args(["status", "--porcelain"]).output() {
            Ok(output) => output,
            Err(_) => {
                self.status_message =
                    String::from("Error: Git command failed. Are you in a git repository?");
                return;
            }
        };

        let output_str = String::from_utf8_lossy(&output.stdout);
        self.files = output_str.lines().map(String::from).collect();

        // もしファイルがない場合は選択状態をリセット
        if self.files.is_empty() {
            self.files_state = ListState::default();
        } else if self.files_state.selected().is_none() {
            self.files_state.select(Some(0));
        }
    }

    fn next(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.files_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    fn stage_file(&mut self) {
        if let Some(i) = self.files_state.selected() {
            if i < self.files.len() {
                let file_status = &self.files[i];
                // status行からパスを適切に抽出
                if let Some(path_start) = file_status.find(' ') {
                    let file_path = file_status[path_start..].trim();

                    // 状態によってコマンドを選択
                    let (cmd, is_staged) =
                        if file_status.starts_with("M ") || file_status.starts_with("A ") {
                            ("reset", true)
                        } else if file_status.starts_with(" M") || file_status.starts_with("??") {
                            ("add", false)
                        } else {
                            self.status_message = format!("Unknown file status: {}", file_status);
                            return;
                        };

                    match Command::new("git").args([cmd, "--", file_path]).output() {
                        Ok(_) => {
                            self.status_message = format!(
                                "{} file: {}",
                                if is_staged { "Unstaged" } else { "Staged" },
                                file_path
                            );
                            self.refresh_files();
                        }
                        Err(e) => {
                            self.status_message = format!("Failed to {} file: {}", cmd, e);
                        }
                    }
                }
            }
        }
    }

    fn commit(&mut self) {
        if self.commit_message.trim().is_empty() {
            self.status_message = String::from("Commit message cannot be empty");
            return;
        }

        match Command::new("git")
            .args(["commit", "-m", &self.commit_message])
            .output()
        {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                self.status_message = result.to_string();
                self.commit_message.clear();
                self.input_mode = InputMode::Normal;
                self.refresh_files();
            }
            Err(_) => {
                self.status_message = String::from("Failed to commit");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--version" => {
                println!("pretty-git-ui version {}", VERSION);
                return Ok(());
            }
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            _ => {
                println!("Unknown option: {}", args[1]);
                print_help();
                return Ok(());
            }
        }
    }

    // ターミナルのセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリの実行
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // 後処理
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn print_help() {
    println!("pretty-git-ui - A simple terminal UI for git");
    println!("Usage: pretty-git-ui [OPTIONS]");
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version information");
    println!("\nKeyboard shortcuts:");
    println!("  q              Quit");
    println!("  j/k or ↓/↑    Navigate files");
    println!("  s              Stage/unstage selected file");
    println!("  c              Enter commit mode");
    println!("  r              Refresh file list");
}

/// イベントループで画面描画、入力処理、状態更新を行う
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // タイムアウト計算
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // イベントのポーリング
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    // 通常モードのキー処理
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') => app.next(),
                        KeyCode::Char('k') => app.previous(),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('s') => app.stage_file(),
                        KeyCode::Char('c') => {
                            app.input_mode = InputMode::Commit;
                        }
                        KeyCode::Char('r') => app.refresh_files(),
                        _ => {}
                    },
                    // コミットモードのキー処理
                    InputMode::Commit => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter => {
                            app.commit();
                        }
                        KeyCode::Char(c) => {
                            app.commit_message.push(c);
                        }
                        KeyCode::Backspace => {
                            app.commit_message.pop();
                        }
                        _ => {}
                    },
                }
            }
        }

        // tickを更新
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

/// レイアウトの管理
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // レイアウトの分割
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

    // 上部ステータスバー
    let status = Paragraph::new(Spans::from(vec![
        Span::raw("Git TUI - Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit, "),
        Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to stage/unstage, "),
        Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to commit, "),
        Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to refresh changed files"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, chunks[0]);

    // ファイルリスト
    let files: Vec<ListItem> = app
        .files
        .iter()
        .map(|i| {
            ListItem::new(i.clone()).style(Style::default().fg(
                if i.starts_with("M ") || i.starts_with("A ") {
                    Color::Green
                } else {
                    Color::Red
                },
            ))
        })
        .collect();

    let files = List::new(files)
        .block(
            Block::default()
                .title("Changed Files")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(files, chunks[1], &mut app.files_state);

    // 下部の入力/ステータスエリア
    match app.input_mode {
        InputMode::Normal => {
            let status_msg = Paragraph::new(app.status_message.clone())
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_msg, chunks[2]);
        }
        InputMode::Commit => {
            let input = Paragraph::new(app.commit_message.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title("Commit Message")
                        .borders(Borders::ALL),
                );
            f.render_widget(input, chunks[2]);
            // カーソルを適切な位置に
            f.set_cursor(
                chunks[2].x + app.commit_message.len() as u16 + 1,
                chunks[2].y + 1,
            );
        }
    }
}
