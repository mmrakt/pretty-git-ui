mod app;
mod git;
mod ui;
mod ui_help;

use app::{App, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::render_ui;

const VERSION: &str = "0.1.0";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--version" => {
                println!("pretty-git-ui version {VERSION}");
                return Ok(());
            },
            "-h" | "--help" => {
                print_help();
                return Ok(());
            },
            _ => {
                println!("Unknown option: {}", args[1]);
                print_help();
                return Ok(());
            },
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
        println!("{err:?}");
    }

    Ok(())
}

fn print_help() {
    println!("pretty-git-ui - A beautiful terminal UI for Git");
    println!("\nUsage: pretty-git-ui [OPTIONS]");
    println!("\nOptions:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version information");
    println!("\nKeyboard shortcuts:");
    println!("  q              Quit application");
    println!("  j/k or ↓/↑    Navigate files");
    println!("  s              Stage/unstage selected file");
    println!("  a              Stage/unstage all files");
    println!("  c              Enter commit mode");
    println!("  t              Enter stash message mode");
    println!("  l              List stashes");
    println!("  p              Apply latest stash");
    println!("  r              Refresh file list");
    println!("  d              Show diff preview (fullscreen)");
    println!("  v              Toggle preview panel");
    println!("\nIn commit/stash mode:");
    println!("  Enter          Submit");
    println!("  Esc            Cancel");
    println!("\nIn preview mode:");
    println!("  j/k or ↓/↑    Scroll preview");
    println!("  q/Esc          Exit preview");
    println!("\nWith preview panel:");
    println!("  Shift+j/k      Scroll preview panel");
    println!("  v              Toggle preview panel");
}

/// イベントループで画面描画、入力処理、状態更新を行う
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|f| render_ui(f, &mut app))?;

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
                        KeyCode::Char('j') | KeyCode::Down => {
                            if app.show_preview_panel {
                                // Check if Shift is held for preview scroll
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::SHIFT)
                                {
                                    app.scroll_preview_down();
                                } else {
                                    app.next();
                                }
                            } else {
                                app.next();
                            }
                        },
                        KeyCode::Char('k') | KeyCode::Up => {
                            if app.show_preview_panel {
                                // Check if Shift is held for preview scroll
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::SHIFT)
                                {
                                    app.scroll_preview_up();
                                } else {
                                    app.previous();
                                }
                            } else {
                                app.previous();
                            }
                        },
                        KeyCode::Char('s') => app.stage_file(),
                        KeyCode::Char('a') => app.stage_all_files(),
                        KeyCode::Char('c') => {
                            app.input_mode = InputMode::Commit;
                        },
                        KeyCode::Char('t') => {
                            app.input_mode = InputMode::StashMessage;
                        },
                        KeyCode::Char('l') => app.list_stashes(),
                        KeyCode::Char('p') => app.apply_latest_stash(),
                        KeyCode::Char('r') => app.refresh_files(),
                        KeyCode::Char('h') => app.show_help(),
                        KeyCode::Char('d') => app.show_preview(),
                        KeyCode::Char('v') => app.toggle_preview_panel(),
                        _ => {},
                    },
                    // コミットモードのキー処理
                    InputMode::Commit => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        },
                        KeyCode::Enter => {
                            app.commit();
                        },
                        KeyCode::Char(c) => {
                            app.commit_message.push(c);
                        },
                        KeyCode::Backspace => {
                            app.commit_message.pop();
                        },
                        _ => {},
                    },
                    // スタッシュメッセージモードのキー処理
                    InputMode::StashMessage => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.stash_message.clear();
                        },
                        KeyCode::Enter => {
                            app.stash_changes();
                        },
                        KeyCode::Char(c) => {
                            app.stash_message.push(c);
                        },
                        KeyCode::Backspace => {
                            app.stash_message.pop();
                        },
                        _ => {},
                    },
                    // Confirm mode key processing
                    InputMode::Confirm { .. } => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            app.handle_confirm(true);
                        },
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.handle_confirm(false);
                        },
                        _ => {},
                    },
                    // Help mode key processing
                    InputMode::Help => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('h') => {
                            app.exit_help();
                        },
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_help_down();
                        },
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_help_up();
                        },
                        _ => {},
                    },
                    // Preview mode key processing (fullscreen)
                    InputMode::Preview { .. } => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.exit_preview();
                        },
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_preview_down();
                        },
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_preview_up();
                        },
                        _ => {},
                    },
                }
            }
        }

    }
}
