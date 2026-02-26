# Getting Started with JetCrab

Welcome to JetCrab v0.4.0! This guide will help you get started with the modern JavaScript runtime.

## Quick Start

1. **[Installation](./installation.md)** - Set up your development environment
2. **[Architecture Overview](../architecture/engine-overview.md)** - Understand how JetCrab works

## What is JetCrab?

JetCrab is a modern JavaScript runtime implemented in Rust, powered by the Chitin (WASM) JavaScript engine and integrated with Tokio for asynchronous operations. It provides a complete JavaScript execution environment with built-in APIs for I/O, networking, and system operations.

## Key Features

### Core Runtime
- **JavaScript Execution**: Full JavaScript execution via Chitin engine
- **Built-in APIs**: Console, Process, and Fetch APIs
- **Async Operations**: Tokio integration for asynchronous I/O
- **CLI Interface**: Command-line tools for running and evaluating JavaScript

### Development Tools
- **Package Management**: CPM package manager for dependency management
- **Hot Reload**: Development server with automatic reloading
- **Linting**: Code quality and style checking
- **Testing**: Built-in testing framework

## Your First JetCrab Program

### 1. Create a JavaScript File
```javascript
// hello.js
console.log("Hello, JetCrab!");
console.log("Version:", process.version);
console.log("Current directory:", process.cwd());
```

### 2. Run the Program
```bash
# Run the JavaScript file
jetcrab run hello.js

# Or evaluate code directly
jetcrab eval "console.log('Hello, JetCrab!'); 42 + 8"
```

### 3. Expected Output
```
Hello, JetCrab!
Version: v18.0.0
Current directory: /path/to/your/project
```

## Working with Packages

### Initialize a Project
```bash
# Create a new project
cpm init my-project
cd my-project
```

### Install Packages
```bash
# Install JavaScript packages
cpm install lodash

# Install Rust crates
cpm install serde

# Install both types
cpm install react serde
```

### Create a Package Configuration
```json
{
  "name": "my-project",
  "version": "0.4.0",
  "main": "src/index.js",
  "dependencies": {
    "lodash": "^4.17.21"
  },
  "rust_dependencies": {
    "serde": "1.0"
  }
}
```

## Development Workflow

### 1. Development Server
```bash
# Start development server with hot reload
cpm dev

# Or with file watching
cpm dev --watch
```

### 2. Code Quality
```bash
# Lint your code
cpm lint

# Format your code
cpm format

# Run tests
cpm test
```

### 3. Building for Production
```bash
# Create production bundle
cpm bundle

# Build optimized version
cargo build --release
```

## API Examples

### Console API
```javascript
console.log("Info message");
console.error("Error message");
console.warn("Warning message");
console.info("Information message");
```

### Process API
```javascript
// Access command line arguments
console.log("Arguments:", process.argv);

// Access environment variables
console.log("Node environment:", process.env.NODE_ENV);

// Get current working directory
console.log("Current directory:", process.cwd());

// Get runtime version
console.log("Version:", process.version);
```

### Fetch API
```javascript
// Make HTTP requests
fetch("https://api.github.com/users/octocat")
  .then(response => response.json())
  .then(data => {
    console.log("User:", data.login);
    console.log("Followers:", data.followers);
  })
  .catch(error => {
    console.error("Fetch error:", error);
  });
```

## Project Structure

```
my-project/
├── src/
│   ├── index.js          # Main entry point
│   └── lib.rs           # Rust library (optional)
├── tests/
│   └── test.js          # Test files
├── package.json            # Package configuration
└── README.md            # Project documentation
```

## Advanced Features

### WebAssembly Integration
```javascript
// Import Rust functions compiled to WASM
import { add, multiply } from './pkg/my_rust_lib.js';

console.log("2 + 3 =", add(2, 3));
console.log("4 * 5 =", multiply(4, 5));
```

### Async Operations
```javascript
// Use async/await with fetch
async function fetchUserData(username) {
  try {
    const response = await fetch(`https://api.github.com/users/${username}`);
    const user = await response.json();
    return user;
  } catch (error) {
    console.error("Error fetching user:", error);
  }
}

// Call the async function
fetchUserData("octocat").then(user => {
  console.log("User data:", user);
});
```

## Best Practices

### 1. Project Organization
- Keep JavaScript and Rust code in separate directories
- Use clear naming conventions
- Organize code into logical modules

### 2. Error Handling
```javascript
// Always handle errors properly
try {
  const result = await riskyOperation();
  console.log("Success:", result);
} catch (error) {
  console.error("Error:", error.message);
}
```

### 3. Performance
- Use async operations for I/O
- Leverage Rust for performance-critical code
- Profile your applications regularly

## Troubleshooting

### Common Issues

#### Module Not Found
```bash
# Check if package is installed
cpm build

# Reinstall if needed
cpm install [package-name]
```

#### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build

# Check for dependency issues
cargo tree
```

#### Runtime Errors
```bash
# Enable debug logging
RUST_LOG=debug jetcrab run your-file.js

# Check for syntax errors
jetcrab eval "your-code-here"
```

## Next Steps

1. **Explore [Examples](../../examples/)** - See more complex examples
2. **Read [Architecture Documentation](../architecture/engine-overview.md)** - Understand the internals
3. **Check [API Reference](../../src/)** - Explore available APIs
4. **Join the [Community](https://github.com/JetCrabCollab/JetCrab/discussions)** - Get help and contribute

## Resources

- **Documentation**: [docs/](../README.md)
- **GitHub Repository**: [JetCrab](https://github.com/JetCrabCollab/JetCrab)
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/JetCrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/JetCrab/discussions)

---

**JetCrab v0.4.0** - Modern JavaScript Runtime in Rust