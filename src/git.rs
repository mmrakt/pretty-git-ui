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
            .map_err(|e| {
                format!(
                    "Failed to run git status: {}. Are you in a git repository?",
                    e
                )
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git status failed: {}", error.trim()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.lines().map(String::from).collect())
    }

    pub fn stage_file(file_status: &str) -> Result<String, String> {
        if file_status.len() < 3 {
            return Err("Invalid file status format".to_string());
        }

        // Git status format: XY filename where X and Y are status codes
        let chars: Vec<char> = file_status.chars().collect();
        if chars.len() < 3 {
            return Err("Invalid file status format".to_string());
        }

        let status_chars: String = chars.iter().take(2).collect();
        let file_path: String = chars.iter().skip(2).collect::<String>().trim().to_string();

        // Check if file is staged (first character is not space)
        let is_staged = !status_chars.chars().next().unwrap_or(' ').is_whitespace();
        let cmd = if is_staged { "reset" } else { "add" };

        let output = Command::new("git")
            .args([cmd, "--", &file_path])
            .output()
            .map_err(|e| format!("Failed to {} file: {}", cmd, e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git {} failed: {}", cmd, error.trim()));
        }

        Ok(format!(
            "✓ {} file: {}",
            if is_staged { "Unstaged" } else { "Staged" },
            &file_path
        ))
    }

    pub fn stage_all_files(files: &[String]) -> Result<String, String> {
        // Check if any file is unstaged (first character is space)
        let has_unstaged = files
            .iter()
            .any(|f| f.len() >= 2 && f.chars().next().unwrap_or(' ').is_whitespace());

        if has_unstaged {
            let output = Command::new("git")
                .args(["add", "."])
                .output()
                .map_err(|e| format!("Failed to stage all files: {}", e))?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Git add failed: {}", error.trim()));
            }
            Ok("✓ All files staged".to_string())
        } else {
            let output = Command::new("git")
                .args(["reset"])
                .output()
                .map_err(|e| format!("Failed to unstage all files: {}", e))?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Git reset failed: {}", error.trim()));
            }
            Ok("✓ All files unstaged".to_string())
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

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            if error.contains("No stash entries found") || error.contains("No stash found") {
                return Ok("No stash to apply".to_string());
            }
            return Err(format!("Failed to apply stash: {}", error.trim()));
        }
        Ok("✓ Latest stash applied successfully".to_string())
    }

    pub fn commit(message: &str) -> Result<String, String> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| format!("Failed to commit: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            if error.contains("nothing to commit") {
                return Ok("Nothing to commit (no staged changes)".to_string());
            }
            return Err(format!("Commit failed: {}", error.trim()));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        if result.contains("create mode")
            || result.contains("delete mode")
            || result.contains("file changed")
        {
            Ok(format!("✓ Committed successfully!\n{}", result.trim()))
        } else {
            Ok("✓ Committed successfully!".to_string())
        }
    }

    pub fn get_current_branch() -> Result<String, String> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .map_err(|e| format!("Failed to get branch: {}", e))?;

        if !output.status.success() {
            return Ok("(no branch)".to_string());
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(if branch.is_empty() {
            "(detached HEAD)".to_string()
        } else {
            branch
        })
    }

    pub fn get_repo_name() -> Result<String, String> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .map_err(|e| format!("Failed to get repo path: {}", e))?;

        if !output.status.success() {
            return Ok("(no repository)".to_string());
        }

        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(path.split('/').last().unwrap_or("repository").to_string())
    }

    pub fn get_file_diff(file_path: &str) -> Result<String, String> {
        // First try to get diff for tracked files
        let output = Command::new("git")
            .args(["diff", "HEAD", "--", file_path])
            .output()
            .map_err(|e| format!("Failed to get diff: {}", e))?;

        if output.status.success() {
            let diff = String::from_utf8_lossy(&output.stdout);
            if !diff.trim().is_empty() {
                return Ok(diff.to_string());
            }
        }

        // If no diff from HEAD, try staged vs working directory
        let output = Command::new("git")
            .args(["diff", "--", file_path])
            .output()
            .map_err(|e| format!("Failed to get working diff: {}", e))?;

        if output.status.success() {
            let diff = String::from_utf8_lossy(&output.stdout);
            if !diff.trim().is_empty() {
                return Ok(diff.to_string());
            }
        }

        // If still no diff, try to show file content for untracked files
        let output = Command::new("cat")
            .arg(file_path)
            .output()
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            Ok(format!("New file content:\n{}", content))
        } else {
            Ok("No changes to preview".to_string())
        }
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
        assert!(!staged_file.chars().next().unwrap().is_whitespace());

        let unstaged_file = " M test.txt";
        assert!(unstaged_file.chars().next().unwrap().is_whitespace());

        let added_file = "A  test.txt";
        assert!(!added_file.chars().next().unwrap().is_whitespace());
    }

    #[test]
    fn test_stage_all_files_detection() {
        let _git_ops = GitOperations::new();

        let all_staged = ["M  file1.txt".to_string(), "A  file2.txt".to_string()];
        let has_unstaged = all_staged
            .iter()
            .any(|f| f.len() >= 2 && f.chars().next().unwrap_or(' ').is_whitespace());
        assert!(!has_unstaged);

        let mixed_files = ["M  file1.txt".to_string(), " M file2.txt".to_string()];
        let has_unstaged = mixed_files
            .iter()
            .any(|f| f.len() >= 2 && f.chars().next().unwrap_or(' ').is_whitespace());
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
