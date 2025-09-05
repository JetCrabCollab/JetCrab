# Hello World - JetCrab Example

This example demonstrates the basic usage of JetCrab runtime with a simple "Hello World" application.

## Features Demonstrated

- Basic JavaScript execution
- Console API usage
- Process API access
- Project initialization with Claw

## Getting Started

### 1. Initialize the Project

```bash
# Navigate to the example directory
cd examples/hello-world

# Initialize a new JetCrab project
claw init

# Install dependencies (if any)
claw install
```

### 2. Run the Example

```bash
# Run the main JavaScript file
jetcrab run index.js

# Or run with specific arguments
jetcrab run index.js --arg "Hello from command line"
```

### 3. Development Mode

```bash
# Start development server with hot reload
claw dev

# Run tests
claw test

# Build for production
claw build
```

## Project Structure

```
hello-world/
├── README.md           # This file
├── package.json        # Project configuration
├── Cargo.toml         # Rust dependencies
├── index.js           # Main JavaScript entry point
├── src/               # Rust source code (if any)
│   └── lib.rs
└── tests/             # Test files
    └── test.js
```

## What This Example Shows

1. **Basic JavaScript Execution**: Simple console output
2. **Process API**: Access to command line arguments and environment
3. **Console API**: Logging with different levels
4. **Project Management**: Using Claw for dependency management

## Next Steps

- Try modifying `index.js` to experiment with different JavaScript features
- Add Rust modules in `src/lib.rs` and compile to WebAssembly
- Explore more advanced examples in the `examples/` directory
