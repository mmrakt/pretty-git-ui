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
- `h`: Show inline help
- `q`: Quit application
- `j/k` or `↓/↑`: Navigate files
- `s`: Stage/unstage selected file
- `a`: Stage/unstage all files (with confirmation for large operations)
- `c`: Enter commit mode
- `t`: Enter stash message mode
- `l`: List stashes
- `p`: Apply latest stash
- `r`: Refresh file list
- `d`: Show diff preview (fullscreen)
- `v`: Toggle preview panel

### Input Modes
- **Normal Mode**: Navigate and stage files
- **Commit Mode**: Enter commit message (Enter to submit, Esc to cancel)
- **Stash Mode**: Enter stash message (Enter to submit, Esc to cancel)
- **Confirm Mode**: Confirm bulk operations (y/n)
- **Preview Mode**: View file diffs fullscreen (j/k to scroll, q/Esc to exit)
- **Preview Panel**: Side-by-side real-time preview (Shift+j/k to scroll, v to toggle)

### Command Line Interface
- `--help/-h`: Show comprehensive help information
- `--version/-v`: Show version information

### UI Improvements
- **Repository info**: Shows current branch and repository name
- **File status indicators**: Clear labels like [staged], [modified], [untracked]
- **Operation feedback**: Success indicators (✓) and clear error messages
- **File count display**: Shows number of changed files
- **Smart confirmations**: Asks for confirmation on bulk operations (>5 files)

## Development Notes

- **Code Quality**: Strict clippy configuration with pedantic lints enabled
- **Testing**: Comprehensive unit and integration tests (64+ tests total)
- **Error Handling**: Enhanced error handling with descriptive messages
- **Performance**: Static methods used where possible to avoid unnecessary allocations
- **UI Updates**: Real-time updates based on git status changes
- **Git Compatibility**: Requires a git repository to function properly
- **User Experience**: Improved feedback, confirmations, and help system

## Recent Improvements

### Major UX Overhaul (v0.1.1)

#### User Experience Improvements
- 🎌 **Complete Japanese Localization**: All UI elements, messages, and help text translated to Japanese
- 🎨 **Simplified Interface Design**: Removed complex ASCII art and decorative elements for better readability
- 📖 **Modular Help System**: Created dedicated `ui_help.rs` module with categorized Japanese help
- ⚡ **Performance Optimization**: Removed unnecessary animations, matrix effects, and frame counting
- 🎯 **Clean File Display**: Eliminated hexadecimal prefixes and simplified file list presentation

#### Technical Improvements
- 🔧 **Code Modularity**: Separated help rendering into dedicated module for maintainability
- 🛡️ **Unicode Safety**: Maintained all existing Unicode character boundary safety features
- 🚀 **Reduced Complexity**: Simplified render functions and removed animation-related code
- 📦 **Library Structure**: Updated module exports and dependencies

#### UI Components Enhanced
- **Status Bar**: Clean, minimal design with essential information only
- **File List**: Simple numbering with clear Japanese status indicators
- **Help System**: Comprehensive Japanese help with logical categorization
- **Input Areas**: All prompts and titles converted to Japanese
- **Preview Panel**: Simplified titles and better readability

### Previous Features (v0.1.0)

#### Fixed Issues
- ✅ Git status parsing for all file state formats
- ✅ Missing keyboard shortcuts in help documentation
- ✅ Error handling and user feedback
- ✅ UI consistency and visual indicators
- ✅ Unicode character boundary safety fixes

#### Enhanced Features
- 🆕 Inline help system (press 'h')
- 🆕 Repository and branch information display
- 🆕 Smart confirmation dialogs for bulk operations
- 🆕 **Real-time diff preview** - Automatic side panel + fullscreen mode
- 🆕 **Enhanced help system** - Categorized, full-screen help interface
- 🆕 Enhanced file status formatting with clear labels
- 🆕 Success indicators and improved error messages
- 🆕 Better visual feedback throughout the application

## Code Patterns

- Git commands executed through static helper functions with comprehensive error handling
- UI rendering separated into focused functions for each panel
- State transitions managed through the enhanced `InputMode` enum (includes Confirm mode)
- File status parsing follows git's porcelain format specifications with robust error handling
- Modern Rust patterns: `?` operator, `match` expressions, iterator chains
- User experience patterns: confirmation dialogs, inline help, visual feedback
- Error recovery: graceful handling of git command failures with user-friendly messages