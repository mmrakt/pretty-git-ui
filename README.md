# Pretty Git UI

A beautiful terminal-based Git user interface built with Rust, providing an intuitive and efficient way to manage your Git workflow.

## Features

- **Interactive File Management**: Stage and unstage files with simple keyboard shortcuts
- **Real-time Git Status**: Live updates of your repository's current state
- **Commit Interface**: Built-in commit message editor with instant feedback
- **Stash Management**: Create, list, and apply stashes seamlessly
- **Keyboard Navigation**: Efficient navigation without leaving your terminal
- **Color-coded Status**: Visual indicators for staged, unstaged, and untracked files

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
| `q` | Quit application |
| `j/k` or `↓/↑` | Navigate files |
| `s` | Stage/unstage selected file |
| `a` | Stage/unstage all files |
| `c` | Enter commit mode |
| `t` | Enter stash message mode |
| `l` | List stashes |
| `p` | Apply latest stash |
| `r` | Refresh file list |

### Command Line Options

```bash
pretty-git-ui --help     # Show help information
pretty-git-ui --version  # Show version information
```

## Screenshots

The interface features a clean three-panel layout:
- **Status Bar**: Shows current mode and application information
- **File List**: Displays files with color-coded status indicators
- **Input Area**: Interactive area for commit messages and commands

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

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Terminal UI powered by [tui-rs](https://github.com/fdehau/tui-rs)
- Cross-platform terminal support via [crossterm](https://github.com/crossterm-rs/crossterm)
