# JetCrab

<div align="center">
  <img src="assets/logo.png" alt="JetCrab Logo" width="200" height="200">
  
  <h1>JetCrab</h1>
  
  <p>A modern JavaScript runtime implemented in Rust, powered by the Boa JavaScript engine and integrated with Tokio for asynchronous operations.</p>
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
  [![Boa Engine](https://img.shields.io/badge/boa-engine-blue.svg)](https://github.com/boa-dev/boa)
  [![Tokio](https://img.shields.io/badge/tokio-async-green.svg)](https://tokio.rs/)
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/JetCrabCollab/jetcrab)
  [![Coverage](https://img.shields.io/badge/coverage-85%25-brightgreen.svg)](https://codecov.io/gh/JetCrabCollab/jetcrab)
  [![Discord](https://img.shields.io/badge/discord-join-7289da.svg)](https://discord.gg/jetcrab)
  [![GitHub Issues](https://img.shields.io/github/issues/JetCrabCollab/jetcrab.svg)](https://github.com/JetCrabCollab/jetcrab/issues)
  [![GitHub Stars](https://img.shields.io/github/stars/JetCrabCollab/jetcrab.svg)](https://github.com/JetCrabCollab/jetcrab/stargazers)
</div>

## Overview

JetCrab is a JavaScript runtime that provides a complete execution environment with built-in APIs for I/O, networking, and system operations. Built on top of the Boa JavaScript engine and integrated with Tokio for asynchronous operations, JetCrab offers a modern alternative to Node.js with Rust's performance and safety guarantees.

## Features

### Core Runtime
- **JavaScript Execution**: Full JavaScript execution via Boa engine
- **Built-in APIs**: Console, Process, and Fetch APIs
- **Async Operations**: Tokio integration for asynchronous I/O
- **CLI Interface**: Command-line tools for running and evaluating JavaScript
- **Package Management**: Claw package manager for dependency management

### Development Tools
- **Hot Reload**: Development server with automatic reloading
- **Linting**: Code quality and style checking
- **Formatting**: Automatic code formatting
- **Testing**: Built-in testing framework
- **Debugging**: Source mapping and debugging support

### Integration
- **WebAssembly**: Rust/JavaScript interoperability via WASM
- **Module System**: ES Modules and CommonJS support
- **Package Registries**: NPM and Cargo registry support
- **Cross-platform**: Windows, macOS, and Linux support

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo package manager

### Build from Source
```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab
cargo build --release
```

### Install Binary
```bash
cargo install jetcrab
```

### Quick Install Scripts
```bash
# Linux/macOS (installs both jetcrab and claw)
curl -sSL https://raw.githubusercontent.com/JetCrabCollab/jetcrab/main/scripts/install.sh | bash

# Windows PowerShell (installs both jetcrab and claw)
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/JetCrabCollab/jetcrab/main/scripts/install.ps1" | Invoke-Expression
```

## Documentation

📚 **Complete documentation is available in the [docs/](docs/) directory:**

- [Installation Guide](docs/INSTALL.md) - Install JetCrab on any platform
- [Distribution Strategy](docs/DISTRIBUTION.md) - How JetCrab is distributed
- [Documentation Index](docs/README.md) - All available documentation

## Quick Start

### JetCrab - JavaScript Runtime
```bash
# Run a JavaScript file
jetcrab run examples/console_test.js

# Evaluate JavaScript code directly
jetcrab eval "console.log('Hello, JetCrab!'); 42 + 8"

# Start interactive REPL
jetcrab repl
```

### Claw - Package Manager
```bash
# Initialize a new project
claw init my-project

# Install packages
claw install react lodash

# Start development server
claw dev
```

### Example JavaScript Code
```javascript
// Console API
console.log("Hello, JetCrab!");
console.error("Error message");
console.warn("Warning message");

// Process API
console.log("Version:", process.version);
console.log("Current directory:", process.cwd());
console.log("Arguments:", process.argv);

// Fetch API
fetch("https://api.github.com/users/octocat")
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error("Fetch error:", error));
```

## Package Management

JetCrab includes **Claw**, a modern package manager that provides unified dependency management for both JavaScript and Rust packages:

```bash
# Initialize a new project
claw init my-project
cd my-project

# Install JavaScript packages
claw install react lodash

# Install Rust crates
claw install serde tokio

# Start development server
claw dev
```

### Claw Features
- **Unified Package Management**: Install JavaScript and Rust packages seamlessly
- **WebAssembly Integration**: Automatic compilation of Rust code to WASM
- **Multi-Registry Support**: NPM, Cargo, and custom registries
- **Development Tools**: Hot reload, linting, formatting, and testing

For complete documentation, see the [Claw Package Manager Guide](docs/guides/claw-package-manager.md).

## Architecture

JetCrab follows a layered architecture:

1. **JavaScript Layer**: User code with standard Web/Node.js APIs
2. **JetCrab Runtime Layer**: API implementations and event loop management
3. **Boa Engine Layer**: JavaScript parsing, AST, and execution
4. **Tokio Async Layer**: Asynchronous I/O operations and task management

This separation ensures that Boa focuses on JavaScript execution while JetCrab manages runtime services and Tokio handles asynchronous operations.

## Boa Integration

JetCrab leverages the Boa JavaScript engine as its core execution engine. This integration provides:

- **ECMAScript Compliance**: Full JavaScript specification support
- **Performance**: Optimized JavaScript execution
- **Reliability**: Battle-tested engine with extensive test coverage
- **Maintenance**: Active development and community support

We contribute improvements and optimizations back to the Boa project, ensuring the entire ecosystem benefits from our enhancements.

## API Reference

### Console API
- `console.log(...args)` - Log messages to stdout
- `console.error(...args)` - Log error messages to stderr
- `console.warn(...args)` - Log warning messages
- `console.info(...args)` - Log info messages

### Process API
- `process.argv` - Command line arguments
- `process.env` - Environment variables
- `process.version` - Runtime version
- `process.cwd()` - Current working directory

### Fetch API
- `fetch(url, options)` - HTTP requests with Promise support
- Response methods: `response.text()`, `response.json()`

## Development

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security vulnerabilities
cargo audit
```

### Documentation
```bash
# Generate documentation
cargo doc --open

# Generate API documentation
cargo doc --no-deps
```

## Contributing

We welcome contributions to JetCrab. Please see our [Contributing Guide](CONTRIBUTING.md) for details on how to get started.

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Code Standards
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features
- Ensure code passes clippy checks
- Maintain backward compatibility

## Performance

JetCrab is designed for performance with the following characteristics:

- **Startup Time**: < 10ms for basic initialization
- **Memory Usage**: < 50MB baseline
- **Execution Speed**: Optimized for common JavaScript patterns
- **Async Performance**: Efficient task spawning and I/O operations

## Security

JetCrab prioritizes security through:

- **Memory Safety**: Rust's ownership system prevents memory-related vulnerabilities
- **Type Safety**: Strong typing prevents runtime type errors
- **Sandboxing**: Planned isolation for untrusted code execution
- **Dependency Management**: Secure package installation and verification

## Roadmap

### Short Term (3-6 months)
- Advanced optimizations and performance improvements
- Enhanced error messages and debugging tools
- File system API implementation
- Module system completion

### Medium Term (6-12 months)
- JIT compilation for performance optimization
- Advanced WebAssembly integration
- Comprehensive testing framework
- Production deployment features

### Long Term (12+ months)
- Multi-threading support
- Advanced security features
- Plugin system architecture
- Enterprise-grade features

## License

JetCrab is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/JetCrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/JetCrab/discussions)
- **Security**: [Security Policy](SECURITY.md)

## Acknowledgments

JetCrab is built on top of several excellent open-source projects:

- [Boa](https://github.com/boa-dev/boa) - JavaScript engine in Rust
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
- [wasm-pack](https://github.com/rustwasm/wasm-pack) - WebAssembly packaging tool

---

**JetCrab** - Modern JavaScript Runtime in Rust