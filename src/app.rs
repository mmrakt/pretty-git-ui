use crate::git::GitOperations;
use tui::widgets::ListState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Commit,
    StashMessage,
    Confirm {
        message: String,
        action: ConfirmAction,
    },
    Preview {
        content: String,
        file_path: String,
    },
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmAction {
    StageAll,
    UnstageAll,
}

#[derive(Debug)]
pub struct App {
    pub files: Vec<String>,
    pub files_state: ListState,
    pub input_mode: InputMode,
    pub commit_message: String,
    pub stash_message: String,
    pub status_message: String,
    pub current_branch: String,
    pub repo_name: String,
    pub preview_scroll: u16,
    pub preview_content: String,
    pub show_preview_panel: bool,
    pub help_scroll: u16,
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
            status_message: String::from(
                "準備完了。[h]でヘルプ、[j/k]でファイル移動できます",
            ),
            current_branch: GitOperations::get_current_branch()
                .unwrap_or_else(|_| "unknown".to_string()),
            repo_name: GitOperations::get_repo_name().unwrap_or_else(|_| "repository".to_string()),
            preview_scroll: 0,
            preview_content: String::new(),
            show_preview_panel: true,
            help_scroll: 0,
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
        // Also refresh branch info
        self.current_branch =
            GitOperations::get_current_branch().unwrap_or_else(|_| "unknown".to_string());
        self.update_preview();
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
        self.update_preview();
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
        self.update_preview();
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

        // Check if we need confirmation
        let has_unstaged = self
            .files
            .iter()
            .any(|f| f.len() >= 2 && f.chars().next().unwrap_or(' ').is_whitespace());

        if has_unstaged && self.files.len() > 5 {
            // Many files to stage, ask for confirmation
            self.input_mode = InputMode::Confirm {
                message: format!("Stage all {} files? (y/n)", self.files.len()),
                action: ConfirmAction::StageAll,
            };
        } else if !has_unstaged && self.files.len() > 5 {
            // Many files to unstage, ask for confirmation
            self.input_mode = InputMode::Confirm {
                message: format!("Unstage all {} files? (y/n)", self.files.len()),
                action: ConfirmAction::UnstageAll,
            };
        } else {
            // Proceed without confirmation for small numbers of files
            self.execute_stage_all();
        }
    }

    fn execute_stage_all(&mut self) {
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

    pub fn show_help(&mut self) {
        self.input_mode = InputMode::Help;
        self.help_scroll = 0;
    }

    pub fn exit_help(&mut self) {
        self.input_mode = InputMode::Normal;
        self.help_scroll = 0;
    }

    pub fn scroll_help_up(&mut self) {
        if self.help_scroll > 0 {
            self.help_scroll -= 1;
        }
    }

    pub fn scroll_help_down(&mut self) {
        self.help_scroll += 1;
    }

    pub fn handle_confirm(&mut self, confirmed: bool) {
        if let InputMode::Confirm { action, .. } = &self.input_mode {
            if confirmed {
                match action {
                    ConfirmAction::StageAll | ConfirmAction::UnstageAll => {
                        self.execute_stage_all();
                    },
                }
            } else {
                self.status_message = String::from("Operation cancelled");
            }
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn show_preview(&mut self) {
        if let Some(i) = self.files_state.selected() {
            if i < self.files.len() {
                let file_status = &self.files[i];
                let chars: Vec<char> = file_status.chars().collect();
                if chars.len() >= 3 {
                    let file_path: String =
                        chars.iter().skip(2).collect::<String>().trim().to_string();
                    match GitOperations::get_file_diff(&file_path) {
                        Ok(content) => {
                            self.input_mode = InputMode::Preview {
                                content,
                                file_path: file_path.to_string(),
                            };
                            self.preview_scroll = 0;
                        },
                        Err(e) => {
                            self.status_message = format!("Preview error: {}", e);
                        },
                    }
                }
            }
        } else {
            self.status_message = String::from("No file selected for preview");
        }
    }

    pub fn scroll_preview_up(&mut self) {
        if self.preview_scroll > 0 {
            self.preview_scroll -= 1;
        }
    }

    pub fn scroll_preview_down(&mut self) {
        self.preview_scroll += 1;
    }

    pub fn exit_preview(&mut self) {
        self.input_mode = InputMode::Normal;
        self.preview_scroll = 0;
    }

    pub fn update_preview(&mut self) {
        if !self.show_preview_panel {
            return;
        }

        if let Some(i) = self.files_state.selected() {
            if i < self.files.len() {
                let file_status = &self.files[i];
                let chars: Vec<char> = file_status.chars().collect();
                if chars.len() >= 3 {
                    let file_path: String =
                        chars.iter().skip(2).collect::<String>().trim().to_string();
                    match GitOperations::get_file_diff(&file_path) {
                        Ok(content) => {
                            self.preview_content = content;
                        },
                        Err(_) => {
                            self.preview_content = "No preview available".to_string();
                        },
                    }
                } else {
                    self.preview_content = "Invalid file status".to_string();
                }
            } else {
                self.preview_content = String::new();
            }
        } else {
            self.preview_content = String::new();
        }
        self.preview_scroll = 0;
    }

    pub fn toggle_preview_panel(&mut self) {
        self.show_preview_panel = !self.show_preview_panel;
        if self.show_preview_panel {
            self.update_preview();
        }
    }

    pub fn get_current_file_path(&self) -> Option<String> {
        if let Some(i) = self.files_state.selected() {
            if i < self.files.len() {
                let file_status = &self.files[i];
                let chars: Vec<char> = file_status.chars().collect();
                if chars.len() >= 3 {
                    return Some(chars.iter().skip(2).collect::<String>().trim().to_string());
                }
            }
        }
        None
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
        assert!(app.status_message.contains("SYSTEM_INIT"));
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
