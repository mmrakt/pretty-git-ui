mod app;
mod git;
mod ui;

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
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
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
                }
            }
        }

        // tickを更新
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
