# Development tools for pretty-git-ui

.PHONY: help build test check fmt clippy clean install run dev

# Default target
help:
	@echo "Available targets:"
	@echo "  build    - Build the project"
	@echo "  test     - Run all tests"
	@echo "  check    - Run cargo check"
	@echo "  fmt      - Format code with rustfmt"
	@echo "  clippy   - Run clippy linter"
	@echo "  clean    - Clean build artifacts"
	@echo "  install  - Install the binary locally"
	@echo "  run      - Run the application"
	@echo "  dev      - Run full development check (fmt + clippy + test)"

# Build the project
build:
	cargo build

# Build release version
build-release:
	cargo build --release

# Run all tests
test:
	cargo test

# Run cargo check
check:
	cargo check

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with fixes
clippy-fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty

# Clean build artifacts
clean:
	cargo clean

# Install locally
install:
	cargo install --path .

# Run the application
run:
	cargo run

# Run with arguments
run-help:
	cargo run -- --help

# Full development check
dev: fmt clippy test
	@echo "✅ All development checks passed!"

# Quick check (no tests)
quick: fmt clippy check
	@echo "✅ Quick checks passed!"

# Pre-commit hook
pre-commit: fmt-check clippy test
	@echo "✅ Pre-commit checks passed!"