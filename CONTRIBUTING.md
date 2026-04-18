# Contributing to ini-editor

Thank you for your interest in contributing!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/ini-editor.git
cd ini-editor

# Build
cd inedit-cli
cargo build --release

# Run tests
cargo test

# Run with lints
cargo clippy
cargo fmt
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Use meaningful variable and function names
- Add comments for complex logic

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and lints
5. Commit with clear messages
6. Push to your fork
7. Submit a Pull Request

## Reporting Bugs

Please include:
- Rust version (`rustc --version`)
- OS details
- Steps to reproduce
- Expected vs actual behavior

## Feature Requests

Open an issue with:
- Clear description
- Use case
- Potential implementation approach