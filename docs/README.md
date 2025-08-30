# JetCrab Documentation

Welcome to the JetCrab documentation! This directory contains comprehensive information about the JetCrab JavaScript engine project.

## 📚 **Documentation Overview**

### **Getting Started**
- **[Getting Started](./getting-started/)** - Installation, setup, and first steps
- **[Beginner's Guide](./guides/beginners-guide.md)** - Comprehensive introduction for newcomers

### **Architecture & Design**
- **[Engine Overview](./architecture/engine-overview.md)** - High-level system architecture
- **[Module Architecture](./architecture/module-architecture.md)** - **UPDATED** - Module organization and design

### **Implementation Status**
- **[Implementation Status](./implementation/implementation-status.md)** - **UPDATED** - Real implementation status
- **[Implementation Overview](./implementation/README.md)** - **UPDATED** - Implementation overview

### **Development & Contributing**
- **[Contributing Guidelines](../CONTRIBUTING.md)** - How to contribute to the project
- **[Code of Conduct](../CODE_OF_CONDUCT.md)** - Community standards and guidelines

## 🚧 **Current Project Status**

**JetCrab is currently in active development with basic functionality working:**

### ✅ **What Works**
- Basic JavaScript execution (arithmetic, strings, variables, objects, arrays)
- Function definitions and calls
- Object and array operations
- Basic error handling
- Math functions (Math.pow, etc.)
- Core compilation pipeline (lexer, parser, bytecode generation)

### 🔄 **In Development**
- Advanced semantic analysis
- Complete test suite (many tests currently failing)
- Performance optimizations
- Advanced memory management
- API stability

### ❌ **Not Yet Implemented**
- Full ECMAScript compliance
- Advanced debugging tools
- Production deployment features
- Comprehensive error recovery
- Module system
- Event system

## 🚨 **Important Notes**

- **Basic functionality works** - You can run simple JavaScript code
- **Some tests are failing** - Due to ongoing refactoring and development
- **API is evolving** - Interfaces may change as development continues
- **Documentation is being updated** - To reflect actual implementation status

## 🎯 **Immediate Priorities**

### **Phase 1: Stabilization (Next 2-4 weeks)**
1. **Fix failing tests** - Update test imports and fix API compatibility
2. **Complete basic features** - Finish semantic analyzer and error handling
3. **Stabilize API** - Stop breaking changes to working features

### **Phase 2: Core Completion (Next 1-2 months)**
1. **Complete core features** - Semantic analysis, error handling, modules
2. **Improve quality** - Better testing, error handling, memory management
3. **Performance optimization** - Basic optimization passes

## 🚀 **Getting Started**

### **Quick Start**
```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd jetcrab
cargo build
cargo run --example basic_usage
```

### **Development Setup**
```bash
cargo test          # Note: Some tests may currently fail
cargo run --example basic_usage  # This works!
cargo build --release
```

## 📊 **Project Metrics**

- **Lines of Code**: ~15,000
- **Working Features**: Basic JavaScript execution
- **Test Status**: Many failing, needs fixing
- **API Stability**: Evolving, needs stabilization
- **Documentation**: Being updated for accuracy

## 🔗 **External Resources**

- **[GitHub Repository](https://github.com/JetCrabCollab/JetCrab)** - Source code and issues
- **[Crates.io](https://crates.io/crates/jetcrab)** - Published package
- **[API Documentation](https://docs.rs/jetcrab)** - Generated API docs

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guidelines](../CONTRIBUTING.md) for details.

### **Current Focus Areas**
1. **Test fixes** - Help fix failing tests
2. **Core features** - Complete semantic analysis and error handling
3. **Documentation** - Keep docs in sync with implementation
4. **Code quality** - Remove warnings and improve structure

## 📝 **Documentation Guidelines**

1. **Follow the structure** - Use existing patterns and organization
2. **Keep it updated** - Ensure documentation matches the current codebase
3. **Use Mermaid diagrams** - Visual explanations where helpful
4. **Cross-reference** - Link to related documentation
5. **Test links** - Verify all internal links work correctly
6. **Be accurate** - Don't overstate implementation status

## 📈 **Project Status**

- **Current Version**: 0.1.0
- **Status**: Basic implementation working, core features in development
- **ECMAScript Compliance**: ~20% (basic features implemented)
- **Next Priority**: Complete core features and fix failing tests

## 🆘 **Need Help?**

- Check the [Beginner's Guide](./guides/beginners-guide.md) for detailed explanations
- Review [Architecture Documentation](./architecture/) for technical details
- Look at [Implementation Status](./implementation/) for current progress
- Join our community discussions for additional support

---

**Note**: This documentation is being updated to accurately reflect the current implementation status. Some sections may still contain outdated information that will be corrected soon. 