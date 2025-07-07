# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust terminal-based Git UI application called `pretty-git-ui` that provides an interactive interface for common Git operations. The application uses the TUI pattern with crossterm and tui crates for terminal manipulation.

## Development Commands

### Build & Run
```bash
# Check compilation without building
cargo check

# Build and run in debug mode
cargo run

# Build optimized release version
cargo build --release

# Run with command line arguments
cargo run -- --help
cargo run -- --version
```

### Code Quality
```bash
# Lint code with Clippy
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt --check
```

### Testing
```bash
# Run tests
cargo test

# Run specific test patterns
cargo test <pattern>
```

## Architecture Overview

### Core Structure
- **Single-file application**: Main logic in `src/main.rs` (~500 lines)
- **MVC pattern**: App state management with terminal UI rendering
- **Event-driven**: Main event loop handles keyboard input and UI updates

### Key Components

1. **App State (`App` struct)**
   - Manages file list, UI state, input buffers, and status messages
   - Handles three input modes: Normal, Commit, StashMessage

2. **Git Integration**
   - Direct git command execution via `std::process::Command`
   - Parses `git status --porcelain` output for file status
   - Supports: status, add, reset, commit, stash operations

3. **UI Layout**
   - Three-panel terminal interface: status bar, file list, input area
   - Color-coded file status (green for staged, red for unstaged)
   - Real-time keyboard input handling

### Dependencies
- **crossterm v0.25**: Cross-platform terminal manipulation
- **tui v0.19**: Terminal User Interface library

## Application Features

### Keyboard Shortcuts
- `q`: Quit application
- `j/k` or `↓/↑`: Navigate files
- `s`: Stage/unstage selected file
- `a`: Stage/unstage all files
- `c`: Enter commit mode
- `t`: Enter stash message mode
- `l`: List stashes
- `p`: Apply latest stash
- `r`: Refresh file list

### Command Line Interface
- `--help/-h`: Show help information
- `--version/-v`: Show version information

## Development Notes

- The codebase includes Japanese comments for maintainability
- Error handling is comprehensive throughout the application
- UI updates happen in real-time based on git status changes
- The application requires a git repository to function properly

## Code Patterns

- Git commands are executed through helper functions that handle error cases
- UI rendering is separated into focused functions for each panel
- State transitions are managed through the `InputMode` enum
- File status parsing follows git's porcelain format specifications
