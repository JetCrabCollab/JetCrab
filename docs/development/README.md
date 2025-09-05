# JetCrab Development

This directory contains development-related documentation for JetCrab.

## 📚 Development Documentation

### Building & Distribution
- **[Building from Source](building.md)** - How to build JetCrab from source code
- **[Distribution Strategy](distribution.md)** - How JetCrab is distributed across platforms

### Development Setup
- **[Development Environment](environment.md)** - Setting up your development environment
- **[Testing Guide](testing.md)** - How to run and write tests
- **[Debugging Guide](debugging.md)** - Debugging JetCrab applications

### Contributing
- **[Contributing Guidelines](../contributing.md)** - How to contribute to JetCrab
- **[Code Style Guide](code-style.md)** - Coding standards and conventions
- **[Release Process](release.md)** - How releases are created

## 🚀 Quick Development Setup

### Prerequisites
- Rust 1.70+ (stable)
- Cargo
- Git

### Build from Source
```bash
git clone https://github.com/JetCrabCollab/jetcrab.git
cd jetcrab
cargo build --release
```

### Run Tests
```bash
cargo test
cargo test --test integration
cargo test --test unit
```

### Development Commands
```bash
# Check code quality
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Run with debug logging
RUST_LOG=debug cargo run --bin jetcrab run examples/hello-world/index.js

# Build for specific platform
cargo build --release --target x86_64-unknown-linux-gnu
```

## 🏗️ Architecture

JetCrab follows a layered architecture:

1. **JavaScript Layer**: User code with standard Web/Node.js APIs
2. **JetCrab Runtime Layer**: API implementations and event loop management
3. **Boa Engine Layer**: JavaScript parsing, AST, and execution
4. **Tokio Async Layer**: Asynchronous I/O operations and task management

## 📝 Development Guidelines

### Code Standards
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features
- Ensure code passes clippy checks
- Maintain backward compatibility

### Testing Strategy
- Unit tests for individual modules
- Integration tests for API functionality
- Performance tests for critical paths
- End-to-end tests for complete workflows

### Performance Considerations
- Use `cargo bench` for performance testing
- Profile with `cargo flamegraph`
- Monitor memory usage with `cargo valgrind`
- Optimize hot paths identified through profiling

## 🔗 Related Documentation

- **[Architecture Overview](../architecture/README.md)** - System design and architecture
- **[Implementation Status](../implementation/README.md)** - Current development status
- **[API Reference](../guides/api-reference.md)** - Public interface details
- **[Contributing Guide](../contributing.md)** - How to contribute

---

**JetCrab Development** - Building the future of JavaScript runtimes 🦀
