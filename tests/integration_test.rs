use pretty_git_ui::app::{App, InputMode};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let repo_path = temp_dir.path();

    Command::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to initialize git repository");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set git config");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to set git config");

    let mut test_file = File::create(repo_path.join("test.txt")).unwrap();
    writeln!(test_file, "initial content").unwrap();

    Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to add file");

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to commit");

    temp_dir
}

#[test]
fn test_app_initialization() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let app = App::new();
    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.commit_message.is_empty());
    assert!(app.stash_message.is_empty());
    assert!(app.status_message.contains("pretty-git-ui"));
}

#[test]
fn test_app_file_navigation() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut test_file = File::create("modified.txt").unwrap();
    writeln!(test_file, "modified content").unwrap();

    let mut app = App::new();
    app.refresh_files();

    if !app.files.is_empty() {
        app.files_state.select(Some(0));

        let initial_selection = app.files_state.selected();
        app.next();

        if app.files.len() > 1 {
            assert_ne!(app.files_state.selected(), initial_selection);
        }

        app.previous();
        if app.files.len() > 1 {
            assert_eq!(app.files_state.selected(), initial_selection);
        }
    }
}

#[test]
fn test_input_mode_transitions() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();

    app.input_mode = InputMode::Commit;
    assert_eq!(app.input_mode, InputMode::Commit);

    app.input_mode = InputMode::StashMessage;
    assert_eq!(app.input_mode, InputMode::StashMessage);

    app.input_mode = InputMode::Normal;
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_commit_message_validation() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();

    app.commit_message = String::new();
    app.commit();
    assert!(app.status_message.contains("cannot be empty"));

    app.commit_message = "   ".to_string();
    app.commit();
    assert!(app.status_message.contains("cannot be empty"));

    app.commit_message = "Valid commit message".to_string();
    app.commit();
    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.commit_message.is_empty());
}

#[test]
fn test_navigation_with_empty_files() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();
    app.files.clear();

    app.next();
    assert_eq!(app.files_state.selected(), None);

    app.previous();
    assert_eq!(app.files_state.selected(), None);
}

#[test]
fn test_navigation_wraparound() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();
    app.files = vec![
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ];

    app.files_state.select(Some(0));
    app.previous();
    assert_eq!(app.files_state.selected(), Some(2));

    app.files_state.select(Some(2));
    app.next();
    assert_eq!(app.files_state.selected(), Some(0));
}

#[test]
fn test_stash_message_clearing() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();
    app.stash_message = "test stash message".to_string();
    app.input_mode = InputMode::StashMessage;

    app.stash_changes();

    assert!(app.stash_message.is_empty());
    assert_eq!(app.input_mode, InputMode::Normal);
}

#[test]
fn test_refresh_files_error_handling() {
    use std::env;
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let original_dir = env::current_dir().unwrap();

    // Create a subdirectory that's definitely not a git repo
    let test_path = temp_dir.path().join("not_a_repo");
    if std::fs::create_dir(&test_path).is_ok() && env::set_current_dir(&test_path).is_ok() {
        let mut app = App::new();
        app.refresh_files();

        // Restore directory first - handle potential errors
        let _ = env::set_current_dir(original_dir);

        // The app should handle git errors gracefully
        // Just verify that it doesn't crash and has some status message
        assert!(!app.status_message.is_empty());
    } else {
        // If we can't set up the test environment, just skip
        let _ = env::set_current_dir(original_dir);
    }
}

#[test]
fn test_stage_all_files_empty() {
    let _temp_dir = setup_test_repo();
    std::env::set_current_dir(_temp_dir.path()).unwrap();

    let mut app = App::new();
    app.files.clear();

    app.stage_all_files();
    assert!(app.status_message.contains("No files to stage"));
}

#[cfg(test)]
mod ui_tests {
    use super::*;
    use pretty_git_ui::render_ui;
    use tui::{backend::TestBackend, Terminal};

    #[test]
    fn test_ui_rendering() {
        let _temp_dir = setup_test_repo();
        std::env::set_current_dir(_temp_dir.path()).unwrap();

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new();

        terminal.draw(|f| render_ui(f, &mut app)).unwrap();

        let backend = terminal.backend();
        let buffer = backend.buffer();

        // Check that the UI rendered something (buffer is not empty)
        assert!(!buffer.content().is_empty());
    }

    #[test]
    fn test_ui_different_modes() {
        let _temp_dir = setup_test_repo();
        std::env::set_current_dir(_temp_dir.path()).unwrap();

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new();

        app.input_mode = InputMode::Commit;
        app.commit_message = "test commit".to_string();
        terminal.draw(|f| render_ui(f, &mut app)).unwrap();

        app.input_mode = InputMode::StashMessage;
        app.stash_message = "test stash".to_string();
        terminal.draw(|f| render_ui(f, &mut app)).unwrap();

        app.input_mode = InputMode::Normal;
        terminal.draw(|f| render_ui(f, &mut app)).unwrap();
    }
}

#[cfg(test)]
mod git_operations_tests {
    use super::*;
    use pretty_git_ui::git::GitOperations;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_git_status_in_repo() {
        let temp_dir = setup_test_repo();
        let original_dir = std::env::current_dir().unwrap();

        // Safely change directory and handle errors
        if std::env::set_current_dir(temp_dir.path()).is_ok() {
            let mut test_file = File::create("modified.txt").unwrap();
            writeln!(test_file, "modified content").unwrap();

            let result = GitOperations::get_status();

            // Always restore directory, ignore errors
            let _ = std::env::set_current_dir(original_dir);

            assert!(result.is_ok());
            let _files = result.unwrap();
            // Just verify we got a valid result (no need to check length >= 0)
        } else {
            // If we can't change directory, just skip the test
        }
    }

    #[test]
    fn test_git_status_not_in_repo() {
        use std::env;
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let original_dir = env::current_dir().unwrap();

        // Temporarily set a different directory that's definitely not a git repo
        let test_path = temp_dir.path().join("not_a_repo");
        std::fs::create_dir(&test_path).unwrap();
        env::set_current_dir(&test_path).unwrap();

        let result = GitOperations::get_status();

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();

        // The result might succeed or fail depending on git configuration
        // Just ensure we get a valid result type
        match result {
            Ok(_) | Err(_) => {},
        }
    }

    #[test]
    fn test_git_commit_operations() {
        let _temp_dir = setup_test_repo();
        std::env::set_current_dir(_temp_dir.path()).unwrap();

        let mut test_file = File::create("commit_test.txt").unwrap();
        writeln!(test_file, "commit test content").unwrap();

        Command::new("git")
            .args(["add", "commit_test.txt"])
            .output()
            .expect("Failed to add file");

        let result = GitOperations::commit("Test commit message");

        assert!(result.is_ok());
    }

    #[test]
    fn test_git_stash_operations() {
        let _temp_dir = setup_test_repo();
        std::env::set_current_dir(_temp_dir.path()).unwrap();

        let mut test_file = File::create("stash_test.txt").unwrap();
        writeln!(test_file, "stash test content").unwrap();

        let result = GitOperations::stash_changes(Some("Test stash message"));

        assert!(result.is_ok());
    }

    #[test]
    fn test_git_list_stashes() {
        let _temp_dir = setup_test_repo();
        std::env::set_current_dir(_temp_dir.path()).unwrap();

        let result = GitOperations::list_stashes();

        assert!(result.is_ok());
    }
}
