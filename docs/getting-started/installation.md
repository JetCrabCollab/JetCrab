# Getting Started with JetCrab

This guide will help you get up and running with the JetCrab JavaScript engine, whether you want to contribute to the project or use it in your own applications.

## Prerequisites

### Required Software
- **Rust**: Version 1.75 or higher
- **Cargo**: Latest stable version (comes with Rust)
- **Git**: For version control
- **Build Tools**: Standard development environment

### Installing Rust
If you don't have Rust installed, follow these steps:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to your PATH
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

## Project Setup

### 1. Clone the Repository
```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab
```

### 2. Build the Project
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release
```

### 3. Run Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test lexer
cargo test parser
cargo test vm
```

### 4. Run Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmarks
cargo bench lexer
```

## Project Structure

### Project Overview
```
jetcrab/
├── Cargo.toml              # Package configuration
├── README.md               # Project overview
├── CONTRIBUTING.md         # Contribution guidelines
├── docs/                   # Documentation
│   ├── README.md          # Documentation index
│   ├── checklists/        # Implementation checklists
│   ├── architecture/      # Architecture documentation
│   ├── development/       # Development guides
│   └── api/               # API documentation
├── src/                    # Source code
│   ├── lib.rs             # Public API and module declarations
│   ├── lexer/             # Lexical analysis
│   ├── ast/               # Abstract Syntax Tree
│   ├── parser/            # Syntax analysis
│   ├── semantic/          # Semantic analysis
│   ├── bytecode/          # Bytecode generation
│   ├── vm/                # Virtual Machine
│   ├── runtime/           # Runtime environment
│   ├── memory/            # Memory management
│   └── api/               # Public API
├── examples/              # Usage examples
├── tests/                 # Integration tests
└── benches/               # Performance benchmarks
```

### Key Directories
- **`src/`**: Source code modules for each component
- **`docs/`**: Comprehensive documentation
- **`examples/`**: Usage examples and demonstrations
- **`tests/`**: Integration tests
- **`benches/`**: Performance benchmarks
- **`target/`**: Build artifacts (generated)

## Quick Examples

### 1. Basic Lexical Analysis
```rust
use v8_lexer::tokenize;

fn main() {
    let source = "let x = 42;";
    match tokenize(source) {
        Ok(tokens) => {
            println!("Found {} tokens:", tokens.len());
            for token in tokens {
                println!("  {:?}", token);
            }
        }
        Err(error) => {
            eprintln!("Lexical error: {}", error);
        }
    }
}
```

### 2. AST Generation and Serialization
```rust
use v8_ast::Node;
use serde_json;

fn main() {
    // Create a simple AST
    let ast = Node::Program(Program {
        body: vec![
            Node::VariableDeclaration(VariableDeclaration {
                declarations: vec![
                    VariableDeclarator {
                        id: Box::new(Node::Identifier("x".to_string())),
                        init: Some(Box::new(Node::Number(42.0))),
                        span: None,
                    }
                ],
                kind: "let".to_string(),
                span: None,
            })
        ],
        source_type: "script".to_string(),
        span: None,
    });

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("AST JSON:\n{}", json);
}
```

### 3. Virtual Machine Execution
```rust
use v8_vm::{Executor, Bytecode, Instruction};
use v8_runtime::Value;

fn main() {
    // Create simple bytecode
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0),  // Push constant 42
        Instruction::Return,         // Return from function
    ]);

    // Execute bytecode
    let mut executor = Executor::new();
    match executor.execute(&bytecode) {
        Ok(value) => {
            println!("Execution result: {:?}", value);
        }
        Err(error) => {
            eprintln!("Execution error: {}", error);
        }
    }
}
```

## Development Workflow

### 1. Making Changes
```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Check formatting
cargo fmt --all

# Run clippy for code quality
cargo clippy --all

# Run tests
cargo test --all

# Commit your changes
git add .
git commit -m "feat(component): add your feature description"
```

### 2. Testing Your Changes
```bash
# Run unit tests
cargo test --all

# Run integration tests
cargo test --test integration_test

# Run benchmarks
cargo bench --all

# Check test coverage (if available)
cargo tarpaulin --all
```

### 3. Documentation
```bash
# Build documentation
cargo doc --all --no-deps

# Open documentation in browser
cargo doc --all --no-deps --open
```

## Common Tasks

### Adding a New Feature
1. **Identify the appropriate crate** for your feature
2. **Add tests** before implementing the feature
3. **Implement the feature** following coding standards
4. **Update documentation** if needed
5. **Run all tests** to ensure nothing breaks

### Debugging Issues
```bash
# Run with debug output
RUST_LOG=debug cargo test --all

# Run specific test with output
cargo test test_name -- --nocapture

# Use rust-gdb for debugging
rust-gdb target/debug/v8_lexer
```

### Performance Profiling
```bash
# Run benchmarks
cargo bench --all

# Profile with perf (Linux)
perf record --call-graph=dwarf cargo bench
perf report

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bench lexer_benchmarks
```

## Configuration

### Environment Variables
- `RUST_LOG`: Set logging level (debug, info, warn, error)
- `RUST_BACKTRACE`: Enable backtraces for debugging
- `CARGO_INCREMENTAL`: Enable incremental compilation

### Cargo Configuration
Create `.cargo/config.toml` for project-specific settings:
```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[profile.dev]
opt-level = 1

[profile.release]
lto = true
codegen-units = 1
```

## Troubleshooting

### Common Issues

#### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build --all

# Update dependencies
cargo update
```

#### Test Failures
```bash
# Run tests with more verbose output
cargo test --all -- --nocapture

# Run specific failing test
cargo test test_name -- --nocapture
```

#### Performance Issues
```bash
# Check for memory leaks
cargo install cargo-valgrind
cargo valgrind test --all

# Profile memory usage
cargo install cargo-instruments
cargo instruments --test
```

### Getting Help

#### Documentation
- **[Project README](../README.md)**: Project overview
- **[Architecture Documentation](../architecture/)**: Technical details
- **[API Documentation](../api/)**: Usage examples
- **[Checklists](../checklists/)**: Implementation status

#### Community
- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Pull Requests**: Contribute code and improvements

#### Resources
- **[Rust Book](https://doc.rust-lang.org/book/)**: Learn Rust
- **[ECMAScript Specification](https://tc39.es/ecma262/)**: JavaScript reference
- **[V8 Engine Documentation](https://v8.dev/)**: V8 engine reference

## Next Steps

### For Contributors
1. Read the **[CONTRIBUTING.md](../CONTRIBUTING.md)** guide
2. Review the **[Architecture Documentation](../architecture/)**
3. Check the **[Implementation Checklists](../checklists/)**
4. Pick an issue or feature to work on
5. Submit a pull request

### For Users
1. Explore the **[API Documentation](../api/)**
2. Try the examples in this guide
3. Check the **[Performance Guide](./performance-guide.md)**
4. Integrate JetCrab into your project

### For Maintainers
1. Review the **[Architecture Documentation](../architecture/)**
2. Monitor the **[Implementation Checklists](../checklists/)**
3. Maintain code quality and test coverage
4. Guide community contributions

---

*This guide should help you get started with JetCrab. For more detailed information, explore the documentation in the `docs/` directory.* 