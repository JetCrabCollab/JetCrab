# Contributing to JetCrab

Thank you for your interest in contributing to JetCrab! This document provides guidelines and information for contributors to help maintain code quality and project consistency.

## Table of Contents
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Continuous Integration](#continuous-integration)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Code Review Process](#code-review-process)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)
- [Performance Guidelines](#performance-guidelines)
- [Security Guidelines](#security-guidelines)

## Getting Started

### Prerequisites
- **Rust**: Version 1.75 or higher
- **Cargo**: Latest stable version
- **Git**: For version control
- **Development Tools**: Standard Rust development environment

### Quick Start
```bash
# Clone the repository
git clone https://github.com/JetCrabCollab/JetCrab.git.git
cd jetcrab

# Build the project
cargo build

# Run all tests
cargo test

# Run benchmarks
cargo bench
```

### Development Environment Setup
```bash
# Install additional development tools
cargo install cargo-watch    # For development with auto-reload
cargo install cargo-tarpaulin # For test coverage
cargo install cargo-audit    # For security audits
cargo install cargo-deny     # For dependency management

# Set up pre-commit hooks (optional)
cargo install cargo-husky
cargo husky install
```

## Development Setup

### Project Structure
JetCrab is organized as a single crate with modular components:

```
jetcrab/
├── src/
│   ├── api/           # Public API and engine interfaces
│   ├── ast/           # Abstract Syntax Tree
│   ├── bytecode/      # Bytecode generation and optimization
│   ├── lexer/         # Lexical analysis
│   ├── memory/        # Memory management and garbage collection
│   ├── parser/        # Syntax analysis
│   ├── runtime/       # Runtime environment
│   ├── semantic/      # Semantic analysis
│   └── vm/            # Virtual Machine
├── tests/             # Integration tests
├── benches/           # Performance benchmarks
├── examples/          # Usage examples
└── docs/              # Documentation
```

### Development Workflow
1. **Create a feature branch**: `git checkout -b feature/your-feature-name`
2. **Make your changes**: Follow coding standards below
3. **Add tests**: Ensure comprehensive test coverage
4. **Run tests**: `cargo test`
5. **Check formatting**: `cargo fmt`
6. **Run clippy**: `cargo clippy`
7. **Run security audit**: `cargo audit`
8. **Check test coverage**: `cargo tarpaulin`
9. **Commit changes**: Follow commit guidelines
10. **Submit PR**: Create pull request with detailed description
11. **CI/CD**: All checks run automatically on GitHub Actions

### Continuous Development
```bash
# Watch for changes and run tests automatically
cargo watch -x test -x clippy -x fmt

# Run tests with coverage
cargo tarpaulin --out Html

# Check for security vulnerabilities
cargo audit

# Validate dependencies
cargo deny check
```

## Continuous Integration

JetCrab uses GitHub Actions for automated testing and quality checks. The CI/CD pipeline runs automatically on every push and pull request.

### What Runs Automatically

#### On Every Push/PR:
- **Formatting Check**: Ensures code follows `rustfmt` standards
- **Linting**: Runs `clippy` to catch common issues and enforce best practices
- **Unit Tests**: Runs all unit tests with `cargo test`
- **Integration Tests**: Runs integration tests
- **Documentation**: Builds documentation to catch doc errors
- **Examples**: Builds all examples to ensure they work
- **Cross-platform Build**: Tests on Ubuntu, Windows, and macOS
- **Security Audit**: Runs `cargo audit` to check for vulnerabilities

#### On Pull Requests Only:
- **TODO/FIXME Check**: Ensures no TODO or FIXME comments are left in code
- **Debug Print Check**: Ensures no debug prints (`println!`, `dbg!`) are left
- **Binary Size Check**: Ensures the binary doesn't exceed size limits

#### On Main Branch Only:
- **Benchmarks**: Runs performance benchmarks
- **Code Coverage**: Generates and uploads coverage reports
- **Release Build**: Creates optimized release builds

### Local Pre-commit Checks

To ensure your code passes CI before pushing, run these commands locally:

```bash
# Format your code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Build examples
cargo build --examples --all-features

# Check documentation
cargo doc --all-features --no-deps

# Security audit
cargo audit

# Check for TODO/FIXME comments
grep -r "TODO\|FIXME" src/ --include="*.rs" || echo "No TODO/FIXME found"

# Check for debug prints
grep -r "println!\|dbg!" src/ --include="*.rs" || echo "No debug prints found"
```

### CI Failure Resolution

If CI fails, check the GitHub Actions logs to understand the issue:

1. **Formatting Issues**: Run `cargo fmt` locally
2. **Clippy Warnings**: Fix the warnings shown in the logs
3. **Test Failures**: Run `cargo test` locally to reproduce
4. **Documentation Errors**: Run `cargo doc` locally
5. **Security Issues**: Run `cargo audit` locally and update dependencies

## Project Structure

### Module Responsibilities

#### **api**
- Public API for embedding JetCrab
- Engine configuration and initialization
- Integration interfaces

#### **ast**
- Abstract Syntax Tree representation
- Serialization and deserialization
- Visitor pattern implementation

#### **bytecode**
- Bytecode generation from AST
- Instruction set definition
- Code optimization

#### **lexer**
- Tokenization of source code
- Unicode support and error handling
- Position tracking and source mapping

#### **memory**
- Memory allocation and management
- Garbage collection algorithms
- Object lifecycle tracking

#### **parser**
- Syntax analysis and AST generation
- Error recovery and reporting
- Language compliance

#### **runtime**
- Runtime value system
- Object and function management
- Context and scope handling
- Built-in functions

#### **semantic**
- Type checking and scope analysis
- Semantic validation
- Error detection and reporting

#### **vm**
- Virtual machine execution
- Stack and register management
- Instruction execution

## Coding Standards

### Rust Code Style
- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` to catch common issues
- Maximum line length: 100 characters

### Naming Conventions
- **Modules**: Use snake_case (`token.rs`, `lexer.rs`)
- **Functions**: Use snake_case (`tokenize`, `parse`)
- **Variables**: Use snake_case (`source_code`, `token_list`)
- **Constants**: Use SCREAMING_SNAKE_CASE (`MAX_TOKENS`, `DEFAULT_BUFFER_SIZE`)
- **Types**: Use PascalCase (`Token`, `Lexer`, `ParseError`)

### Documentation Standards
- **Public APIs**: Must have doc comments (`///`)
- **Modules**: Include module-level documentation (`//!`)
- **Examples**: Provide usage examples in doc comments
- **Error Types**: Document all possible error conditions

### Code Organization
```rust
// 1. Module documentation
//! Module description

// 2. Imports (external first, then internal)
use std::collections::HashMap;
use crate::token::Token;

// 3. Public types and constants
pub struct Lexer {
    // ...
}

// 4. Private types and constants
const DEFAULT_BUFFER_SIZE: usize = 1024;

// 5. Public functions
impl Lexer {
    pub fn new(source: &str) -> Self {
        // ...
    }
}

// 6. Private functions
impl Lexer {
    fn tokenize_number(&mut self) -> Result<Token, LexerError> {
        // ...
    }
}

// 7. Tests (at the bottom of the file)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_creation() {
        // ...
    }
}
```

## Testing Guidelines

### Test Structure
- **Unit Tests**: In the same file as the code being tested
- **Integration Tests**: In `tests/` directory
- **Benchmarks**: In `benches/` directory for performance-critical code

### Test Requirements
- **Coverage**: Aim for 100% test coverage for new code
- **Edge Cases**: Test error conditions and boundary cases
- **Performance**: Include benchmarks for performance-critical code
- **Documentation**: Tests should serve as usage examples
- **Integration**: Test cross-module interactions
- **Regression**: Ensure no performance regressions

### Test Categories
- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test module interactions and end-to-end workflows
- **Property Tests**: Use `proptest` for property-based testing
- **Benchmark Tests**: Performance testing with `criterion`
- **Documentation Tests**: Code examples in doc comments

### Test Naming
- **Unit Tests**: `test_function_name_scenario`
- **Integration Tests**: `test_module_integration_scenario`
- **Benchmarks**: `bench_operation_name`

### Example Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenizes_simple_identifier() {
        let source = "hello";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(_)));
    }

    #[test]
    fn test_lexer_handles_unicode_identifiers() {
        let source = "let π = 3.14;";
        let tokens = tokenize(source).unwrap();
        // Verify Unicode support
    }

    #[test]
    fn test_lexer_reports_errors_appropriately() {
        let source = "\"unterminated string";
        let result = tokenize(source);
        assert!(result.is_err());
    }
}
```

## Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **perf**: Performance improvements
- **ci**: CI/CD changes
- **chore**: Maintenance tasks

### Examples
```
feat(lexer): add Unicode identifier support

- Support Unicode identifiers like π, 你好, 🚀
- Add comprehensive test coverage
- Update documentation with examples

Closes #123
```

```
fix(vm): resolve memory leak in function calls

The function call stack wasn't being properly cleaned up,
causing memory leaks in recursive functions.
```

## Pull Request Process

### Before Submitting
1. **Ensure tests pass**: `cargo test`
2. **Check formatting**: `cargo fmt`
3. **Run clippy**: `cargo clippy`
4. **Run security audit**: `cargo audit`
5. **Check test coverage**: `cargo tarpaulin`
6. **Update documentation**: Add/update relevant docs
7. **Add tests**: Include tests for new functionality
8. **Check for breaking changes**: Ensure API compatibility

### PR Description Template
```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Benchmarks added/updated
- [ ] All tests pass

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or breaking changes documented)

## Related Issues
Closes #123
```

## Code Review Process

### Review Guidelines
- **Be constructive**: Provide helpful feedback
- **Focus on code**: Avoid personal comments
- **Ask questions**: If something is unclear
- **Suggest improvements**: Offer specific suggestions
- **Check for**: Correctness, performance, security, maintainability

### Review Checklist
- [ ] Code follows project standards
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] No performance regressions
- [ ] Error handling is appropriate
- [ ] Security considerations addressed

## Reporting Issues

### Bug Reports
When reporting bugs, please include:
- **Description**: Clear description of the issue
- **Reproduction**: Steps to reproduce the problem
- **Expected vs Actual**: What you expected vs what happened
- **Environment**: Rust version, OS, etc.
- **Code Example**: Minimal code to reproduce the issue

### Issue Template
```markdown
## Bug Description
[Clear description of the bug]

## Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

## Expected Behavior
[What you expected to happen]

## Actual Behavior
[What actually happened]

## Environment
- Rust Version: [e.g., 1.75.0]
- OS: [e.g., Ubuntu 22.04]
- Architecture: [e.g., x86_64]

## Additional Information
[Any other relevant information]
```

## Feature Requests

### Feature Request Guidelines
- **Clear description**: What the feature should do
- **Use case**: Why this feature is needed
- **Implementation ideas**: How it might be implemented
- **Priority**: High/Medium/Low priority

### Feature Request Template
```markdown
## Feature Description
[Clear description of the requested feature]

## Use Case
[Why this feature is needed and how it would be used]

## Proposed Implementation
[Optional: Ideas for how to implement this feature]

## Priority
[High/Medium/Low]

## Additional Information
[Any other relevant information]
```

## Getting Help

### Communication Channels
- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Requests**: For code contributions

### Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [JetCrab Documentation](https://docs.rs/jetcrab/)
- [ECMAScript Specification](https://tc39.es/ecma262/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

## Performance Guidelines

### Benchmarking
- **Always benchmark** performance-critical changes
- **Use criterion** for reliable benchmark results
- **Compare against baselines** to detect regressions
- **Profile with tools** like `perf`, `flamegraph`, or `cargo-instruments`

### Optimization Principles
- **Measure first**: Profile before optimizing
- **Optimize hot paths**: Focus on frequently executed code
- **Consider trade-offs**: Balance performance vs. maintainability
- **Document performance characteristics**: Include benchmarks in documentation

## Security Guidelines

### Code Security
- **Use `cargo audit`** to check for known vulnerabilities
- **Validate all inputs** from external sources
- **Use safe defaults** for configuration
- **Follow Rust security best practices**

### Dependency Management
- **Keep dependencies updated** with `cargo update`
- **Use `cargo deny`** to manage dependency policies
- **Review new dependencies** before adding them
- **Prefer well-maintained crates** with active development

## License

By contributing to JetCrab, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to JetCrab! Your contributions help make this project better for everyone. 