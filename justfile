# Development workflow for pretty-git-ui
# Requires: https://github.com/casey/just

# List available recipes
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run cargo check
check:
    cargo check

# Format code with rustfmt
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all --check

# Run clippy
clippy:
    cargo clippy --all-targets --all-features

# Run clippy with fixes
clippy-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty

# Run clippy with stricter settings
clippy-strict:
    cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Install locally
install:
    cargo install --path .

# Run the application
run:
    cargo run

# Run with help flag
run-help:
    cargo run -- --help

# Run with version flag
run-version:
    cargo run -- --version

# Full development workflow (format + lint + test)
dev: fmt clippy test
    @echo "✅ All development checks passed!"

# Quick pre-commit check
quick: fmt clippy check
    @echo "✅ Quick checks passed!"

# Pre-commit hook workflow
pre-commit: fmt-check clippy-strict test
    @echo "✅ Pre-commit checks passed!"

# Generate documentation
doc:
    cargo doc --open

# Check for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Benchmark (when benchmarks exist)
bench:
    cargo bench

# Run with cargo watch for development
watch:
    cargo watch -x check -x test

# Watch and run on changes
watch-run:
    cargo watch -x run

# Profile build times
build-timings:
    cargo build --timings

# Generate cargo lock diff
lock-diff:
    git diff Cargo.lock