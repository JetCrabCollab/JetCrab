# JetCrab Implementation

This directory contains implementation details and development guidelines for JetCrab.

## Overview

JetCrab v0.4.0 is a functional JavaScript runtime built on top of the Chitin (WASM) JavaScript engine with Tokio integration for asynchronous operations.

## Current Implementation Status

### Completed Features
- **JavaScript Execution**: Fully functional via Chitin engine
- **Built-in APIs**: Console, Process, and Fetch APIs working
- **Async Runtime**: Tokio integration complete
- **CLI Interface**: `jetcrab run`, `jetcrab eval` commands working
- **Package Manager**: CPM structure implemented

### In Development
- **Module System**: ES Modules support in development
- **WebAssembly**: Basic Rust/JS integration structure
- **Development Tools**: Hot reload, linting planned

## Development Guidelines

### Code Standards
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features
- Ensure code passes clippy checks
- Maintain backward compatibility

### Testing
```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test integration
cargo test --test unit

# Run with coverage
cargo tarpaulin
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check for issues
cargo clippy
cargo fmt --check
```

## Architecture

JetCrab follows a layered architecture:

1. **JavaScript Layer**: User code with standard Web/Node.js APIs
2. **JetCrab Runtime Layer**: API implementations and event loop management
3. **Chitin Engine Layer**: JavaScript parsing, AST, and execution
4. **Tokio Async Layer**: Asynchronous I/O operations and task management

## Contributing

See the main [Contributing Guide](../../CONTRIBUTING.md) for details on how to contribute to JetCrab.

---

**JetCrab v0.4.0** - Modern JavaScript Runtime in Rust