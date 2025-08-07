# Contributing to TuxPilot

Thank you for your interest in contributing to TuxPilot! This document provides guidelines and information for contributors.

## Getting Started

### Development Environment

1. **Fork and clone the repository:**
```bash
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot
```

2. **Install Rust and dependencies:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

3. **Install development tools:**
```bash
cargo install cargo-watch cargo-audit cargo-outdated
```

4. **Build and test:**
```bash
cargo build
cargo test
```

## Development Workflow

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Ensure all tests pass: `cargo test`

### Project Structure

```
src/
├── main.rs              # Entry point and CLI parsing
├── cli.rs               # Interactive CLI interface
├── config.rs            # Configuration management
├── ai/                  # AI integration modules
│   └── mod.rs          # AI client abstraction
├── error_diagnosis.rs   # Error detection and analysis
├── linux_integration.rs # System integration
└── system_monitor.rs    # System monitoring
```

### Adding New Features

1. **Create a feature branch:**
```bash
git checkout -b feature/your-feature-name
```

2. **Implement your feature:**
   - Add tests for new functionality
   - Update documentation as needed
   - Follow existing code patterns

3. **Test thoroughly:**
```bash
cargo test
cargo clippy
cargo fmt --check
```

4. **Submit a pull request:**
   - Provide clear description of changes
   - Reference any related issues
   - Ensure CI passes

## Types of Contributions

### Bug Reports

When reporting bugs, please include:
- Operating system and version
- TuxPilot version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

### Feature Requests

For new features:
- Describe the use case
- Explain why it would be valuable
- Consider implementation complexity
- Discuss potential alternatives

### Code Contributions

Areas where contributions are welcome:
- New AI provider integrations
- Additional Linux distribution support
- Enhanced error diagnosis patterns
- System monitoring improvements
- Documentation and examples
- Test coverage improvements

## Specific Contribution Areas

### AI Integration

To add a new AI provider:

1. Extend the `AiProvider` enum in `config.rs`
2. Add configuration structure
3. Implement the provider in `ai/mod.rs`
4. Add tests and documentation

### Linux Distribution Support

To add support for a new package manager:

1. Add to `PackageManager` enum in `config.rs`
2. Implement suggestion methods in `linux_integration.rs`
3. Add detection logic in `Config::detect_system()`
4. Test on the target distribution

### Error Diagnosis

To improve error detection:

1. Add new error patterns in `error_diagnosis.rs`
2. Extend categorization logic
3. Add test cases with real error examples
4. Update documentation

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_test
```

### Writing Tests

- Unit tests: Place in the same file as the code
- Integration tests: Place in `tests/` directory
- Use descriptive test names
- Test both success and error cases

### Test Coverage

Aim for good test coverage, especially for:
- Configuration parsing
- Error diagnosis logic
- System integration functions
- AI provider interactions

## Documentation

### Code Documentation

- Use rustdoc comments (`///`) for public APIs
- Include examples in documentation
- Document error conditions
- Keep documentation up to date

### User Documentation

- Update README.md for new features
- Add examples to demonstrate usage
- Update installation instructions if needed
- Consider adding to the wiki

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Run full test suite
4. Update documentation
5. Create release tag
6. Publish to crates.io (maintainers only)

## Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers learn
- Maintain professional communication

### Communication

- Use GitHub issues for bug reports and feature requests
- Use GitHub discussions for questions and ideas
- Be patient with response times
- Provide context and details

## Getting Help

If you need help with development:

1. Check existing documentation
2. Search through GitHub issues
3. Ask in GitHub discussions
4. Reach out to maintainers

## Recognition

Contributors will be:
- Listed in the project README
- Credited in release notes
- Invited to join the contributors team (for regular contributors)

Thank you for contributing to TuxPilot! 🐧
