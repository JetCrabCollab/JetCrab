# Contributing to JetCrab

Thank you for your interest in contributing to JetCrab! This guide will help you get started with contributing to the project.

## 🤝 How to Contribute

### Types of Contributions
- **Bug Reports**: Report issues you encounter
- **Feature Requests**: Suggest new features or improvements
- **Code Contributions**: Submit pull requests with code changes
- **Documentation**: Improve or add documentation
- **Testing**: Add or improve tests
- **Examples**: Create example applications or tutorials

### Getting Started

1. **Fork the Repository**: Click the "Fork" button on GitHub
2. **Clone Your Fork**: `git clone https://github.com/your-username/jetcrab.git`
3. **Create a Branch**: `git checkout -b feature/your-feature-name`
4. **Make Changes**: Implement your changes
5. **Test Your Changes**: Run tests and ensure everything works
6. **Submit a Pull Request**: Create a PR with a clear description

## 🚀 Development Setup

### Prerequisites
- Rust 1.70+ (stable channel)
- Cargo
- Git
- Make (optional)

### Setup Steps
```bash
# Clone your fork
git clone https://github.com/your-username/jetcrab.git
cd jetcrab

# Add upstream remote
git remote add upstream https://github.com/JetCrabCollab/jetcrab.git

# Build the project
cargo build

# Run tests
cargo test
```

## 📝 Code Standards

### Rust Code Style
- Follow Rust naming conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for issues
- Add documentation for public APIs
- Include tests for new features

### Documentation Standards
- Write clear, concise documentation
- Use proper markdown formatting
- Include examples where appropriate
- Keep documentation up to date

### Commit Message Format
```
type(scope): brief description

Detailed description of changes

Fixes #issue-number
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples**:
```
feat(api): add new console.log method
fix(runtime): resolve memory leak in event loop
docs(guide): update installation instructions
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration

# Run with output
cargo test -- --nocapture
```

### Writing Tests
- Write unit tests for individual functions
- Write integration tests for API functionality
- Test error cases and edge conditions
- Ensure tests are deterministic and fast

### Test Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

## 🔍 Code Quality

### Pre-commit Checks
```bash
# Format code
cargo fmt

# Check with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps
```

### Continuous Integration
All pull requests must pass:
- Rust compilation
- Clippy checks
- Formatting checks
- All tests
- Documentation generation

## 📋 Pull Request Process

### Before Submitting
1. **Sync with upstream**: `git fetch upstream && git rebase upstream/main`
2. **Run all checks**: `cargo fmt && cargo clippy && cargo test`
3. **Update documentation**: If adding new features
4. **Add tests**: For new functionality
5. **Update changelog**: If applicable

### Pull Request Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] New tests added for new functionality
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Changelog updated (if applicable)
```

## 🐛 Bug Reports

### Before Reporting
1. Check if the issue already exists
2. Try the latest version
3. Reproduce the issue
4. Gather relevant information

### Bug Report Template
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g. Windows 10, Ubuntu 20.04]
- Rust version: [e.g. 1.70.0]
- JetCrab version: [e.g. 0.4.0]

**Additional context**
Add any other context about the problem.
```

## 💡 Feature Requests

### Before Requesting
1. Check if the feature already exists
2. Consider if it fits the project's goals
3. Think about implementation complexity
4. Consider alternatives

### Feature Request Template
```markdown
**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
A clear description of any alternative solutions.

**Additional context**
Add any other context or screenshots.
```

## 🏗️ Architecture Guidelines

### Module Organization
- Keep modules focused and cohesive
- Minimize coupling between modules
- Use clear, descriptive names
- Follow established patterns

### API Design
- Design APIs for ease of use
- Maintain backward compatibility
- Document public interfaces
- Consider performance implications

### Error Handling
- Use appropriate error types
- Provide helpful error messages
- Handle errors gracefully
- Log errors appropriately

## 📚 Documentation

### Code Documentation
- Document all public APIs
- Use rustdoc format
- Include examples in documentation
- Keep documentation up to date

### User Documentation
- Write clear, concise guides
- Include practical examples
- Keep documentation current
- Test documentation examples

## 🎯 Areas for Contribution

### High Priority
- **Performance Improvements**: Optimize critical paths
- **API Completeness**: Implement missing APIs
- **Testing**: Improve test coverage
- **Documentation**: Enhance user guides

### Medium Priority
- **Error Handling**: Improve error messages
- **Logging**: Enhance logging capabilities
- **Examples**: Create more examples
- **Tools**: Improve development tools

### Low Priority
- **Code Style**: Minor style improvements
- **Refactoring**: Code organization improvements
- **Comments**: Improve code comments
- **Minor Features**: Small feature additions

## 🤔 Questions?

### Getting Help
- **GitHub Discussions**: Ask questions in discussions
- **GitHub Issues**: Report bugs or request features
- **Discord**: Join our community Discord
- **Email**: Contact maintainers directly

### Resources
- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Cargo Book**: [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)
- **JetCrab Documentation**: [docs/](docs/)
- **Boa Engine**: [https://boa-dev.github.io/](https://boa-dev.github.io/)

## 📄 License

By contributing to JetCrab, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation
- Community acknowledgments

---

**Thank you for contributing to JetCrab!** 🦀

Together, we're building the future of JavaScript runtimes.
