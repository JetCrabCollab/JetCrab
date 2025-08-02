<img src="assets/logo.png" alt="JetCrab Logo" width="200" />
# JetCrab

A modern JavaScript engine written in Rust, designed for performance, safety, and extensibility.

## Features

- **Lexical Analysis**: Fast tokenization with error recovery
- **Parsing**: Robust AST generation with syntax error handling
- **Semantic Analysis**: Type checking and scope management
- **Bytecode Generation**: Optimized instruction generation
- **Virtual Machine**: Stack-based execution engine
- **Memory Management**: Garbage collection and memory allocation
- **Runtime Environment**: Object system and built-in functions

## Architecture

JetCrab follows a modular architecture with clear separation of concerns:

```
src/
├── lexer/          # Tokenization and lexical analysis
├── ast/            # Abstract Syntax Tree representation
├── parser/         # Syntax analysis and AST construction
├── semantic/       # Type checking and semantic analysis
├── bytecode/       # Code generation and optimization
├── vm/             # Virtual machine and execution
├── runtime/        # Runtime environment and objects
├── memory/         # Memory management and garbage collection
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