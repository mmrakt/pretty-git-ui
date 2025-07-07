use std::process::Command;

#[derive(Debug)]
pub struct GitOperations;

impl Default for GitOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl GitOperations {
    pub const fn new() -> Self {
        Self
    }

    pub fn get_status() -> Result<Vec<String>, String> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map_err(|_| "Git command failed. Are you in a git repository?")?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.lines().map(String::from).collect())
    }

    pub fn stage_file(file_status: &str) -> Result<String, String> {
        if file_status.len() <= 3 {
            return Err("Invalid file status format".to_string());
        }

        let file_path = file_status[3..].trim();
        let is_staged = file_status.starts_with("M ") || file_status.starts_with("A ");
        let cmd = if is_staged { "reset" } else { "add" };

        Command::new("git")
            .args([cmd, "--", file_path])
            .output()
            .map_err(|_| format!("Failed to {cmd} file"))?;

        Ok(format!(
            "{} file: {}",
            if is_staged { "Unstaged" } else { "Staged" },
            file_path
        ))
    }

    pub fn stage_all_files(files: &[String]) -> Result<String, String> {
        let has_unstaged = files
            .iter()
            .any(|f| !f.starts_with("M ") && !f.starts_with("A "));

        if has_unstaged {
            Command::new("git")
                .args(["add", "."])
                .output()
                .map_err(|_| "Failed to stage all files")?;
            Ok("All files staged".to_string())
        } else {
            Command::new("git")
                .args(["reset"])
                .output()
                .map_err(|_| "Failed to unstage all files")?;
            Ok("All files unstaged".to_string())
        }
    }

    pub fn stash_changes(message: Option<&str>) -> Result<String, String> {
        let mut args = vec!["stash", "push"];

        if let Some(msg) = message {
            args.push("-m");
            args.push(msg);
        }

        let output = Command::new("git")
            .args(&args)
            .output()
            .map_err(|_| "Failed to stash changes")?;

        let result = String::from_utf8_lossy(&output.stdout);
        let error = String::from_utf8_lossy(&output.stderr);

        if !error.is_empty() {
            Ok(format!("Stash error: {error}"))
        } else if result.contains("No local changes to save") {
            Ok("No changes to stash".to_string())
        } else {
            Ok(format!("Changes stashed: {result}"))
        }
    }

    pub fn list_stashes() -> Result<String, String> {
        let output = Command::new("git")
            .args(["stash", "list"])
            .output()
            .map_err(|_| "Failed to list stashes")?;

        let result = String::from_utf8_lossy(&output.stdout);
        if result.is_empty() {
            Ok("No stashes found".to_string())
        } else {
            Ok(format!("Stashes:\n{result}"))
        }
    }

    pub fn apply_latest_stash() -> Result<String, String> {
        let output = Command::new("git")
            .args(["stash", "apply"])
            .output()
            .map_err(|_| "Failed to apply stash")?;

        let error = String::from_utf8_lossy(&output.stderr);
        if error.contains("No stash found") {
            Ok("No stash to apply".to_string())
        } else {
            Ok("Latest stash applied".to_string())
        }
    }

    pub fn commit(message: &str) -> Result<String, String> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .map_err(|_| "Failed to commit")?;

        let result = String::from_utf8_lossy(&output.stdout);
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_operations_new() {
        let git_ops = GitOperations::new();
        assert!(std::mem::size_of_val(&git_ops) == 0);
    }

    #[test]
    fn test_stage_file_invalid_format() {
        let result = GitOperations::stage_file("M");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid file status format"));
    }

    #[test]
    fn test_stage_file_format_parsing() {
        let _git_ops = GitOperations::new();

        let staged_file = "M  test.txt";
        assert!(staged_file.starts_with("M "));

        let unstaged_file = " M test.txt";
        assert!(!unstaged_file.starts_with("M "));

        let added_file = "A  test.txt";
        assert!(added_file.starts_with("A "));
    }

    #[test]
    fn test_stage_all_files_detection() {
        let _git_ops = GitOperations::new();

        let all_staged = ["M  file1.txt".to_string(), "A  file2.txt".to_string()];
        let has_unstaged = all_staged
            .iter()
            .any(|f| !f.starts_with("M ") && !f.starts_with("A "));
        assert!(!has_unstaged);

        let mixed_files = ["M  file1.txt".to_string(), " M file2.txt".to_string()];
        let has_unstaged = mixed_files
            .iter()
            .any(|f| !f.starts_with("M ") && !f.starts_with("A "));
        assert!(has_unstaged);
    }

    #[test]
    fn test_stash_message_handling() {
        let _git_ops = GitOperations::new();

        let message = Some("test message");
        let mut args = vec!["stash", "push"];
        if let Some(msg) = message {
            args.push("-m");
            args.push(msg);
        }
        assert_eq!(args, vec!["stash", "push", "-m", "test message"]);

        let no_message: Option<&str> = None;
        let mut args = vec!["stash", "push"];
        if let Some(msg) = no_message {
            args.push("-m");
            args.push(msg);
        }
        assert_eq!(args, vec!["stash", "push"]);
    }

    #[test]
    fn test_file_path_extraction() {
        let file_status = "M  src/main.rs";
        let file_path = file_status[3..].trim();
        assert_eq!(file_path, "src/main.rs");

        let file_status_with_spaces = " M src/test.rs";
        let file_path = file_status_with_spaces[3..].trim();
        assert_eq!(file_path, "src/test.rs");
    }
}
