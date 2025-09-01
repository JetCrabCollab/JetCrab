# Getting Started with JetCrab

Welcome to JetCrab! This directory contains everything you need to get started with the project.

## Quick Start

1. **[Installation](./installation.md)** - Set up your development environment
2. **[Getting Started Overview](./README.md)** - This document

## What is JetCrab?

JetCrab is a modern JavaScript engine written in Rust, designed for performance, safety, and extensibility. It provides a basic JavaScript execution pipeline structure from source code parsing to bytecode execution.

## Current Status

**JetCrab is currently in active development with basic infrastructure in place:**

✅ **Working Infrastructure:**
- Project compiles successfully with warnings
- Basic compilation pipeline structure exists
- Core module architecture implemented
- Basic memory management framework

🔄 **In Development:**
- JavaScript execution engine (currently non-functional)
- Semantic analysis implementation
- Complete test suite (many tests currently failing)
- Performance optimizations
- Advanced memory management

❌ **Not Yet Implemented:**
- Functional JavaScript execution
- Basic arithmetic operations
- String operations
- Variable declarations
- Object and array operations
- Function definitions and calls
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
- **Status**: Basic infrastructure complete, JavaScript execution non-functional
- **ECMAScript Compliance**: 0% (no JavaScript execution)
- **Next Priority**: Fix JavaScript execution engine and implement basic operations

## Need Help?

- Check the [Beginner's Guide](../guides/beginners-guide.md) for detailed explanations
- Review [Architecture Documentation](../architecture/) for technical details
- Look at [Implementation Status](../implementation/) for current progress
- Join our community discussions for additional support

## Important Notes

- **Basic infrastructure exists** - Project structure and compilation pipeline are in place
- **JavaScript execution fails** - All examples fail at execution
- **Many tests are failing** - Due to incomplete implementation
- **API is evolving** - Interfaces may change as development continues
- **Documentation is accurate** - Now reflects actual implementation status

## Getting Started Checklist

- [ ] **Install Rust** - Ensure you have Rust 1.70+ installed
- [ ] **Clone repository** - Get the latest source code
- [ ] **Build project** - Run `cargo build` to compile
- [ ] **Run examples** - Try `cargo run --example fibonacci` (will fail at execution)
- [ ] **Run tests** - Note that many tests currently fail
- [ ] **Explore code** - Look at the infrastructure and basic implementation structure

## Current Limitations

### What You Can Do
- **Compile the project** - `cargo build` works successfully
- **Explore the code structure** - Well-organized Rust modules
- **Understand the architecture** - Clear separation of concerns
- **See the framework** - Basic compilation pipeline structure

### What You Cannot Do
- **Execute JavaScript code** - All execution fails
- **Run working examples** - Examples compile but fail at runtime
- **Use the API** - Public interfaces exist but don't work
- **Run passing tests** - Many tests currently fail

## Next Steps

Once you understand the current limitations:

1. **Contribute to core functionality** - Help fix the JavaScript execution engine
2. **Implement basic operations** - Start with arithmetic and strings
3. **Fix failing tests** - Update tests to match current implementation
4. **Improve code quality** - Remove warnings and improve structure

## Support and Community

- **GitHub Issues**: Report bugs and request features
- **Discussions**: Join community conversations
- **Contributing**: Help improve the project
- **Documentation**: Keep docs accurate and helpful 