# Pretty Git UI

A beautiful and user-friendly terminal-based Git user interface built with Rust, providing an intuitive and efficient way to manage your Git workflow. Now featuring improved UX with Japanese language support and streamlined interface design.

## Features

- **Interactive File Management**: Stage and unstage files with simple keyboard shortcuts
- **Real-time Git Status**: Live updates of your repository's current state
- **Commit Interface**: Built-in commit message editor with instant feedback
- **Stash Management**: Create, list, and apply stashes seamlessly
- **Keyboard Navigation**: Efficient navigation without leaving your terminal
- **Color-coded Status**: Visual indicators for staged, unstaged, and untracked files
- **Japanese Language Support**: Full Japanese localization for improved accessibility
- **Clean Interface**: Streamlined design focused on usability and readability
- **Real-time Preview**: Automatic diff preview with side-by-side layout

## Installation

### Using Homebrew (macOS)

```bash
brew tap mmrakt/pretty-git-ui
brew install pretty-git-ui
```

### From Source

```bash
git clone https://github.com/mmrakt/pretty-git-ui.git
cd pretty-git-ui
cargo install --path .
```

### Using Cargo

```bash
cargo install pretty-git-ui
```

## Usage

Navigate to any Git repository and run:

```bash
pretty-git-ui
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `h` | Show inline help |
| `q` | Quit application |
| `j/k` or `↓/↑` | Navigate files |
| `s` | Stage/unstage selected file |
| `a` | Stage/unstage all files |
| `c` | Enter commit mode |
| `t` | Enter stash message mode |
| `l` | List stashes |
| `p` | Apply latest stash |
| `r` | Refresh file list |
| `d` | Show diff preview (fullscreen) |
| `v` | Toggle preview panel |

#### Input Modes
- **Commit/Stash Mode**: `Enter` to submit, `Esc` to cancel
- **Confirmation Mode**: `y` to confirm, `n` or `Esc` to cancel  
- **Preview Mode**: `j/k` or `↓/↑` to scroll, `q/Esc` to exit
- **Preview Panel**: `Shift+j/k` to scroll preview, `v` to toggle

### Command Line Options

```bash
pretty-git-ui --help     # Show help information
pretty-git-ui --version  # Show version information
```

## Interface

The interface features a clean, user-friendly three-panel layout:
- **Status Bar**: Clean design showing repository name, current branch, and essential shortcuts
- **File List**: Simplified display with clear Japanese status indicators
- **Preview Panel**: Real-time diff preview with Unicode-safe rendering (toggle with `v`)
- **Input Area**: Intuitive Japanese interface for commit messages and status feedback

### UI/UX Improvements
- ✓ **Japanese Localization**: Complete Japanese language support for all UI elements
- ✓ **Simplified Design**: Removed complex ASCII art for better readability
- ✓ **Smart Confirmations**: User-friendly confirmation prompts for bulk operations
- ✓ **Visual Feedback**: Clear success indicators and error messages
- ✓ **Enhanced Help System**: Press `h` for categorized, easy-to-read Japanese help screen
- ✓ **Clean File Status**: Simplified file indicators without hexadecimal prefixes
- ✓ **Auto Preview**: Automatically shows diff when selecting files
- ✓ **Performance Optimized**: Removed unnecessary animations for faster response

## Development

### Prerequisites

- Rust 1.70 or later
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/mmrakt/pretty-git-ui.git
cd pretty-git-ui

# Build and run
cargo run

# Build optimized release
cargo build --release
```

### Development Commands

```bash
# Check code without building
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test
```

## Architecture

Pretty Git UI is built with a clean MVC architecture:

- **Model**: Git repository state and file status tracking
- **View**: Terminal UI rendered with the `tui` crate
- **Controller**: Event handling and state management

### Key Dependencies

- **crossterm**: Cross-platform terminal manipulation
- **tui**: Terminal user interface library

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Homebrew Formula

The Homebrew formula for this project is maintained separately at:
https://github.com/mmrakt/homebrew-pretty-git-ui

## Recent Updates (v0.1.1)

### Major UX Improvements
- 🎌 **Japanese Localization**: Complete Japanese language support for better accessibility
- 🎨 **Simplified Interface**: Removed complex ASCII art and hexadecimal prefixes for cleaner look
- 📖 **Enhanced Help System**: New Japanese help module with categorized keyboard shortcuts
- ⚡ **Performance Optimization**: Removed unnecessary animations and matrix effects
- 🎯 **User-Friendly Design**: Streamlined UI focusing on usability over aesthetics

### Fixed Issues (v0.1.0)
- ✅ Comprehensive git status parsing for all file formats
- ✅ Missing keyboard shortcuts in documentation
- ✅ Enhanced error handling and user feedback
- ✅ UI consistency and visual improvements
- ✅ Unicode character boundary safety fixes

### Previous Features
- 🆕 **Real-time diff preview** (automatic side panel + fullscreen mode)
- 🆕 Repository and branch information display
- 🆕 Smart confirmation dialogs for bulk operations
- 🆕 Enhanced file status formatting
- 🆕 Success indicators and improved error messages

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Terminal UI powered by [tui-rs](https://github.com/fdehau/tui-rs)
- Cross-platform terminal support via [crossterm](https://github.com/crossterm-rs/crossterm)
