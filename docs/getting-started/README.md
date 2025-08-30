# Getting Started with JetCrab

Welcome to JetCrab! This directory contains everything you need to get started with the project.

## Quick Start

1. **[Installation](./installation.md)** - Set up your development environment
2. **[Getting Started Overview](./README.md)** - This document

## What is JetCrab?

JetCrab is a modern JavaScript engine written in Rust, designed for performance, safety, and extensibility. It provides a basic JavaScript execution pipeline from source code parsing to bytecode execution.

## Current Status

**JetCrab is currently in active development with basic functionality working:**

✅ **Working Features:**
- Basic JavaScript execution (arithmetic, strings, variables, objects, arrays)
- Function definitions and calls
- Object and array operations
- Basic error handling
- Math functions (Math.pow, etc.)

🔄 **In Development:**
- Advanced semantic analysis
- Complete test suite
- Performance optimizations
- Advanced memory management

❌ **Not Yet Implemented:**
- Full ECMAScript compliance
- Advanced debugging tools
- Production deployment features
- Comprehensive error recovery

## For Different User Types

### **New to JetCrab?**
Start with the [Beginner's Guide](../guides/beginners-guide.md) for a comprehensive introduction to the project and computer science concepts.

### **Developers**
- Check the [Architecture Overview](../architecture/engine-overview.md) to understand the system design
- Review [Implementation Status](../implementation/implementation-status.md) for current progress
- Explore [Module Architecture](../architecture/module-architecture.md) for technical details

### **Contributors**
- Read [Contributing Guidelines](../CONTRIBUTING.md) for contribution guidelines
- Check [Implementation Status](../implementation/) for current work items
- Review [Architecture Documentation](../architecture/) for system design

## Project Status

- **Current Version**: 0.1.0
- **Status**: Basic implementation working, core features in development
- **ECMAScript Compliance**: ~20% (basic features implemented)
- **Next Priority**: Complete core features and fix failing tests

## Need Help?

- Check the [Beginner's Guide](../guides/beginners-guide.md) for detailed explanations
- Review [Architecture Documentation](../architecture/) for technical details
- Look at [Implementation Status](../implementation/) for current progress
- Join our community discussions for additional support

## Important Notes

- **Basic functionality works** - You can run simple JavaScript code
- **Some tests are failing** - Due to ongoing refactoring and development
- **API is evolving** - Interfaces may change as development continues
- **Documentation is being updated** - To reflect actual implementation status

## Getting Started Checklist

- [ ] **Install Rust** - Ensure you have Rust 1.70+ installed
- [ ] **Clone repository** - Get the latest source code
- [ ] **Build project** - Run `cargo build` to compile
- [ ] **Run examples** - Try `cargo run --example basic_usage`
- [ ] **Run tests** - Note that some tests may currently fail
- [ ] **Explore code** - Look at the working examples and basic implementation 