use crate::git::GitOperations;
use tui::widgets::ListState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Commit,
    StashMessage,
}

#[derive(Debug)]
pub struct App {
    pub files: Vec<String>,
    pub files_state: ListState,
    pub input_mode: InputMode,
    pub commit_message: String,
    pub stash_message: String,
    pub status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            files: Vec::new(),
            files_state: ListState::default(),
            input_mode: InputMode::Normal,
            commit_message: String::new(),
            stash_message: String::new(),
            status_message: String::from("pretty-git-ui v0.1.0 - Welcome"),
        };
        app.refresh_files();
        if !app.files.is_empty() {
            app.files_state.select(Some(0));
        }
        app
    }

    pub fn refresh_files(&mut self) {
        match GitOperations::get_status() {
            Ok(files) => {
                self.files = files;
                if self.files.is_empty() {
                    self.files_state = ListState::default();
                } else if self.files_state.selected().is_none() {
                    self.files_state.select(Some(0));
                }
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }

    pub fn next(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.files_state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.files_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        self.files_state.select(Some(i));
    }

    pub fn stage_file(&mut self) {
        if let Some(i) = self.files_state.selected() {
            if i < self.files.len() {
                let file_status = &self.files[i];
                match GitOperations::stage_file(file_status) {
                    Ok(message) => {
                        self.status_message = message;
                        self.refresh_files();
                    },
                    Err(e) => {
                        self.status_message = format!("Error: {e}");
                    },
                }
            }
        }
    }

    pub fn stage_all_files(&mut self) {
        if self.files.is_empty() {
            self.status_message = String::from("No files to stage");
            return;
        }

        match GitOperations::stage_all_files(&self.files) {
            Ok(message) => {
                self.status_message = message;
                self.refresh_files();
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }

    pub fn stash_changes(&mut self) {
        let message = if self.stash_message.trim().is_empty() {
            None
        } else {
            Some(self.stash_message.as_str())
        };

        match GitOperations::stash_changes(message) {
            Ok(result_message) => {
                self.status_message = result_message;
                self.stash_message.clear();
                self.input_mode = InputMode::Normal;
                self.refresh_files();
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }

    pub fn list_stashes(&mut self) {
        match GitOperations::list_stashes() {
            Ok(message) => {
                self.status_message = message;
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }

    pub fn apply_latest_stash(&mut self) {
        match GitOperations::apply_latest_stash() {
            Ok(message) => {
                self.status_message = message;
                self.refresh_files();
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }

    pub fn commit(&mut self) {
        if self.commit_message.trim().is_empty() {
            self.status_message = String::from("Commit message cannot be empty");
            return;
        }

        match GitOperations::commit(&self.commit_message) {
            Ok(message) => {
                self.status_message = message;
                self.commit_message.clear();
                self.input_mode = InputMode::Normal;
                self.refresh_files();
            },
            Err(e) => {
                self.status_message = format!("Error: {e}");
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.commit_message.is_empty());
        assert!(app.stash_message.is_empty());
        assert!(app.status_message.contains("pretty-git-ui"));
    }

    #[test]
    fn test_input_mode_transitions() {
        let mut app = App::new();

        app.input_mode = InputMode::Commit;
        assert_eq!(app.input_mode, InputMode::Commit);

        app.input_mode = InputMode::StashMessage;
        assert_eq!(app.input_mode, InputMode::StashMessage);

        app.input_mode = InputMode::Normal;
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_navigation_empty_files() {
        let mut app = App::new();
        app.files.clear();
        app.files_state = tui::widgets::ListState::default();

        app.next();
        assert_eq!(app.files_state.selected(), None);

        app.previous();
        assert_eq!(app.files_state.selected(), None);
    }

    #[test]
    fn test_navigation_with_files() {
        let mut app = App::new();
        app.files = vec![
            "file1".to_string(),
            "file2".to_string(),
            "file3".to_string(),
        ];
        app.files_state.select(Some(0));

        app.next();
        assert_eq!(app.files_state.selected(), Some(1));

        app.next();
        assert_eq!(app.files_state.selected(), Some(2));

        app.next();
        assert_eq!(app.files_state.selected(), Some(0));

        app.previous();
        assert_eq!(app.files_state.selected(), Some(2));
    }

    #[test]
    fn test_commit_message_validation() {
        let mut app = App::new();

        app.commit_message = String::new();
        app.commit();
        assert!(app.status_message.contains("cannot be empty"));

        app.commit_message = "   ".to_string();
        app.commit();
        assert!(app.status_message.contains("cannot be empty"));
    }
}
