# Contributing to MatteriaTrack

Thank you for your interest in contributing to MatteriaTrack!

## Code of Conduct

Be respectful and constructive. We're all here to build something cool.

## Getting Started

### Prerequisites

- Rust 1.70+
- Git
- SQLite (for testing)

### Setup

```bash
# Clone repository
git clone https://github.com/ind4skylivey/matteria-track.git
cd matteria-track

# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- track -p "Test" -t "Dev"
```

## How to Contribute

### Reporting Bugs

1. Check existing issues first
2. Use the bug report template
3. Include:
   - MatteriaTrack version (`mtrack --version`)
   - OS and architecture
   - Steps to reproduce
   - Expected vs actual behavior
   - Relevant logs

### Suggesting Features

1. Check existing issues/discussions
2. Describe the use case
3. Explain how it fits MatteriaTrack's vision
4. Be open to feedback

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/awesome-feature`
3. Make your changes
4. Add tests if applicable
5. Run checks: `cargo test && cargo clippy && cargo fmt`
6. Commit with clear message
7. Push and create PR

## Development Guidelines

### Code Style

- Follow Rust idioms
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write descriptive commit messages

### Commit Messages

Format: `[TYPE] Description`

Types:
- `FEAT` - New feature
- `FIX` - Bug fix
- `DOCS` - Documentation
- `REFACTOR` - Code refactoring
- `TEST` - Adding tests
- `CHORE` - Maintenance

Examples:
```
[FEAT] Add pomodoro timer integration
[FIX] Handle timezone edge case in stats
[DOCS] Update installation guide for macOS
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_tracking_engine

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Documentation

- Update docs for new features
- Add doc comments for public APIs
- Include examples in doc comments

```rust
/// Starts tracking time for a project and task.
///
/// # Arguments
///
/// * `project` - The project name
/// * `task` - The task name
///
/// # Example
///
/// ```
/// engine.start_tracking("MyProject", "Development")?;
/// ```
pub fn start_tracking(&mut self, project: &str, task: &str) -> Result<()>
```

## Project Structure

```
src/
├── lib.rs          # Library entry point
├── main.rs         # CLI entry point
├── cli.rs          # Command-line interface
├── config.rs       # Configuration handling
├── database.rs     # SQLite database
├── models.rs       # Data models
├── tracking.rs     # Tracking engine
├── stats.rs        # Statistics calculation
├── ui.rs           # Terminal UI
├── error.rs        # Error types
├── theme.rs        # Theme definitions
├── themes/         # Theme system
├── achievements.rs # Achievement system
├── fuzzy.rs        # Fuzzy finder
├── notifications.rs # Notifications
├── integrations/   # External integrations
└── security/       # Security features
```

## Areas for Contribution

### Good First Issues

- Documentation improvements
- Adding tests
- Small bug fixes
- Typo fixes

### Intermediate

- New achievements
- Theme improvements
- Integration enhancements
- CLI improvements

### Advanced

- New integrations
- Performance optimization
- Security features
- Cross-platform support

## Testing Checklist

Before submitting PR:

- [ ] `cargo test` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt` applied
- [ ] New features have tests
- [ ] Documentation updated
- [ ] CHANGELOG updated (for features)

## Release Process

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
4. Push tag: `git push origin v1.0.0`
5. GitHub Actions handles the rest

## Questions?

- Open a GitHub Discussion
- Check existing issues
- Read the documentation

---

*"Together we forge the ultimate Materia"*
