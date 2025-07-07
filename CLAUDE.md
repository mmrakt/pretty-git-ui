# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust terminal-based Git UI application called `pretty-git-ui` that provides an interactive interface for common Git operations. The application uses the TUI pattern with crossterm and tui crates for terminal manipulation.

## Development Commands

### Quick Development Workflow
```bash
# Full development workflow (format + lint + test)
make dev
# OR with just
just dev

# Quick checks (format + lint + check - no tests)
make quick
# OR
just quick

# Pre-commit checks (format check + strict lint + test)
make pre-commit
# OR
just pre-commit
```

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
cargo clippy --all-targets --all-features

# Format code
cargo fmt --all

# Check formatting without modifying files
cargo fmt --all --check

# Strict clippy (fail on warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Auto-fix clippy issues
cargo clippy --fix --allow-dirty
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test patterns
cargo test <pattern>
```

### Development Tools

The project includes several configuration files for code quality:
- **Cargo.toml**: Clippy lints configuration with strict quality rules
- **.clippy.toml**: Additional clippy configuration
- **rustfmt.toml**: Code formatting rules
- **rust-toolchain.toml**: Rust toolchain specification
- **Makefile**: Development commands
- **justfile**: Alternative task runner (requires [just](https://github.com/casey/just))
- **.git-hooks/pre-commit**: Pre-commit hook template

## Architecture Overview

### Core Structure
- **Modular design**: Separated into app, git, and ui modules
- **MVC pattern**: App state management with terminal UI rendering
- **Event-driven**: Main event loop handles keyboard input and UI updates

### Key Components

1. **App State (`app.rs`)**
   - Manages file list, UI state, input buffers, and status messages
   - Handles three input modes: Normal, Commit, StashMessage
   - Coordinates between git operations and UI updates

2. **Git Integration (`git.rs`)**
   - Static methods for git command execution
   - Parses `git status --porcelain` output for file status
   - Supports: status, add, reset, commit, stash operations
   - Comprehensive error handling and result parsing

3. **UI Layout (`ui.rs`)**
   - Three-panel terminal interface: status bar, file list, input area
   - Color-coded file status (green for staged, red for unstaged)
   - Real-time keyboard input handling and cursor management

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

- **Code Quality**: Strict clippy configuration with pedantic lints enabled
- **Testing**: Comprehensive unit and integration tests (64+ tests total)
- **Error Handling**: All git operations have proper error handling
- **Performance**: Static methods used where possible to avoid unnecessary allocations
- **UI Updates**: Real-time updates based on git status changes
- **Git Compatibility**: Requires a git repository to function properly

## Code Patterns

- Git commands executed through static helper functions with error handling
- UI rendering separated into focused functions for each panel
- State transitions managed through the `InputMode` enum
- File status parsing follows git's porcelain format specifications
- Modern Rust patterns: `?` operator, `match` expressions, iterator chains