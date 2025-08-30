<img src="assets/logo.png" alt="JetCrab Logo" width="200" />

![CI](https://github.com/JetCrabCollab/JetCrab/actions/workflows/ci.yml/badge.svg?branch=main)
![Security](https://github.com/JetCrabCollab/JetCrab/actions/workflows/security.yml/badge.svg?branch=main)
![Coverage](https://github.com/JetCrabCollab/JetCrab/actions/workflows/coverage.yml/badge.svg?branch=main)
![Documentation](https://github.com/JetCrabCollab/JetCrab/actions/workflows/docs.yml/badge.svg?branch=main)

# JetCrab

A modern JavaScript engine written in Rust, designed for performance, safety, and extensibility.

## Features

- **Lexical Analysis**: Fast tokenization with error recovery
- **Parsing**: Robust AST generation with syntax error handling
- **Semantic Analysis**: Basic type checking and scope management
- **Bytecode Generation**: Instruction generation and optimization
- **Virtual Machine**: Stack-based execution engine
- **Memory Management**: Basic memory allocation and management
- **Runtime Environment**: Object system and built-in functions

## Current Status

**JetCrab is currently in active development with core functionality working:**

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

## Architecture

JetCrab follows a modular architecture with clear separation of concerns:

```
src/
├── lexer/          # Tokenization and lexical analysis
├── ast/            # Abstract Syntax Tree representation
├── parser/         # Syntax analysis and AST construction
├── semantic/       # Basic type checking and validation
├── bytecode/       # Code generation and optimization
├── vm/             # Virtual machine and execution
├── runtime/        # Runtime environment and objects
├── memory/         # Basic memory management
└── api/            # Public API and engine interface
```

## Quick Start

### Installation

```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd jetcrab
cargo build
```

### Basic Usage

```rust
use jetcrab::Engine;

fn main() {
    let mut engine = Engine::new();
    
    // Evaluate JavaScript code
    let result = engine.evaluate("2 + 3 * 4");
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

### Running Examples

```bash
cargo run --example basic_usage
```

### Running Tests

```bash
cargo test
```

**Note:** Some tests may currently fail due to ongoing refactoring.

## Development

### Project Structure

- `src/` - Main source code
- `tests/` - Integration and unit tests
- `examples/` - Usage examples
- `benches/` - Performance benchmarks
- `docs/` - Documentation

### Development Tools

#### Automatic Formatting

The project is configured to automatically format code on commit. You can also run formatting manually:

```bash
# Format code
cargo fmt --all

# Check formatting without changing files
cargo fmt --all -- --check
```

#### Code Quality

```bash
# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings

# Run all development checks
make dev

# Or use the development script
./scripts/dev.sh
```

#### Building

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

#### Makefile Commands

```bash
make help          # Show all available commands
make fmt           # Format code
make clippy        # Run linter
make test          # Run tests
make build         # Build project
make dev           # Run all checks
make check         # Check formatting and linting
make bench         # Run benchmarks
make doc           # Generate documentation
make run-examples  # Run all examples
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Fix failing tests and stabilize API
- [ ] Complete semantic analysis implementation
- [ ] Improve error handling and recovery
- [ ] ECMAScript 2024 compliance
- [ ] JIT compilation
- [ ] WebAssembly support
- [ ] Node.js compatibility layer
- [ ] Performance optimizations
- [ ] Debugging tools
- [ ] REPL interface

## Acknowledgments

- Inspired by V8, SpiderMonkey, and other modern JavaScript engines
- Built with Rust for performance and safety
- Community-driven development 