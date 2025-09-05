# JetCrab Guides

This directory contains comprehensive guides and tutorials for using JetCrab.

## Available Guides

### Getting Started
- **[Installation Guide](../getting-started/installation.md)** - Set up your development environment
- **[Quick Start Guide](../getting-started/README.md)** - Your first steps with JetCrab
- **[JetCrab Guide](jetcrab-guide.md)** - Comprehensive guide to using JetCrab

### User Guides
- **[CLI Reference](cli-reference.md)** - Command-line interface documentation
- **[API Reference](api-reference.md)** - Complete API documentation
- **[Examples](examples.md)** - Code examples and tutorials

### Package Management
- **[Claw Package Manager](claw-package-manager.md)** - Complete guide to the package manager

### Architecture
- **[Engine Overview](../architecture/engine-overview.md)** - High-level architecture and design principles

## Learning Path

### For Beginners
1. Start with the [Installation Guide](../getting-started/installation.md)
2. Follow the [Quick Start Guide](../getting-started/README.md)
3. Read the comprehensive [JetCrab Guide](jetcrab-guide.md)
4. Check out [Examples](examples.md) to see JetCrab in action
5. Learn about [Claw Package Manager](claw-package-manager.md)
6. Explore the [Architecture Overview](../architecture/engine-overview.md)

### For Developers
1. Review the [Architecture Documentation](../architecture/engine-overview.md)
2. Read the comprehensive [JetCrab Guide](jetcrab-guide.md)
3. Check the [Implementation Status](../implementation/README.md)
4. Learn about [Claw Package Manager](claw-package-manager.md)
5. Explore [Examples](examples.md) and [API Reference](api-reference.md)
6. Check out the [Examples](../../examples/) directory

### For Contributors
1. Read the [Contributing Guide](../contributing.md)
2. Understand the [Architecture](../architecture/engine-overview.md)
3. Review the [Implementation Guidelines](../implementation/README.md)
4. Check out [Examples](examples.md) and [API Reference](api-reference.md)

## Key Concepts

### JavaScript Runtime
JetCrab is a JavaScript runtime that provides:
- JavaScript execution via Boa engine
- Built-in APIs (console, process, fetch)
- Asynchronous operations via Tokio
- Package management via Claw

### Boa Integration
JetCrab leverages the Boa JavaScript engine for:
- ECMAScript compliance
- Performance optimization
- Reliability and stability
- Active community support

### Async Operations
Tokio integration provides:
- HTTP client capabilities
- File I/O operations
- Timer management
- Task spawning and management

## Examples

### Basic JavaScript Execution
```javascript
console.log("Hello, JetCrab!");
console.log("2 + 3 =", 2 + 3);
```

### Using Built-in APIs
```javascript
// Process API
console.log("Version:", process.version);
console.log("Arguments:", process.argv);

// Fetch API
fetch("https://api.github.com/users/octocat")
  .then(response => response.json())
  .then(data => console.log(data));
```

### Package Management
```bash
# Initialize project
claw init my-project

# Install packages
claw install lodash serde

# Run development server
claw dev
```

## Resources

- **Documentation**: [docs/](../README.md)
- **Examples**: [examples/](../../examples/)
- **GitHub Repository**: [JetCrab](https://github.com/JetCrabCollab/JetCrab)
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/JetCrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/JetCrab/discussions)

---

**JetCrab v0.4.0** - Modern JavaScript Runtime in Rust