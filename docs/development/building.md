# Building JetCrab from Source

This guide explains how to build JetCrab from source code.

## Prerequisites

### Required Tools
- **Rust 1.70+** (stable channel recommended)
- **Cargo** (comes with Rust)
- **Git** (for cloning the repository)
- **Make** (optional, for using Makefile commands)

### Platform-Specific Requirements

#### Windows
- **Visual Studio Build Tools** or **Visual Studio Community**
- **Windows SDK**
- **Git for Windows**

#### Linux
- **Build essentials** (gcc, make, etc.)
- **pkg-config**
- **OpenSSL development libraries**

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# CentOS/RHEL/Fedora
sudo yum groupinstall "Development Tools"
sudo yum install pkgconfig openssl-devel
```

#### macOS
- **Xcode Command Line Tools**
- **Homebrew** (recommended)

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Building JetCrab

### 1. Clone the Repository

```bash
git clone https://github.com/JetCrabCollab/jetcrab.git
cd jetcrab
```

### 2. Build in Debug Mode

```bash
# Build the main binary
cargo build

# Build all binaries (jetcrab + claw)
cargo build --bins

# Build with all features
cargo build --all-features
```

### 3. Build in Release Mode

```bash
# Build optimized release version
cargo build --release

# Build with all features in release mode
cargo build --release --all-features
```

### 4. Build Specific Targets

```bash
# Build only the jetcrab binary
cargo build --bin jetcrab

# Build only the claw binary
cargo build --bin claw

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu
```

## Using the Makefile

JetCrab includes a Makefile for common build tasks:

```bash
# Build everything
make build

# Build release version
make build-release

# Clean build artifacts
make clean

# Run tests
make test

# Check code quality
make check

# Format code
make format
```

## Cross-Compilation

### Building for Different Platforms

#### Linux to Windows
```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

#### Linux to macOS
```bash
# Add macOS target
rustup target add x86_64-apple-darwin

# Build for macOS (requires macOS SDK)
cargo build --release --target x86_64-apple-darwin
```

#### Windows to Linux
```bash
# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

## Feature Flags

JetCrab supports several feature flags:

```bash
# Build with all features
cargo build --all-features

# Build with specific features
cargo build --features "http,fs,console"

# Build without optional features
cargo build --no-default-features
```

### Available Features
- **default**: Core runtime features
- **http**: HTTP client and server APIs
- **fs**: File system APIs
- **console**: Console API
- **process**: Process API
- **timers**: Timer APIs (setTimeout, setInterval)
- **crypto**: Cryptographic APIs
- **wasm**: WebAssembly support
- **dev-tools**: Development tools and debugging

## Development Builds

### Debug Builds
```bash
# Build with debug symbols
cargo build

# Build with debug logging enabled
RUST_LOG=debug cargo build

# Build with specific log level
RUST_LOG=info cargo build
```

### Profile Builds
```bash
# Build with profiling information
cargo build --profile dev

# Build with optimization but debug info
cargo build --profile dev --release
```

## Testing

### Run All Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration
```

### Test Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html

# Run coverage for specific tests
cargo tarpaulin --tests --out Html
```

## Code Quality

### Linting
```bash
# Run clippy
cargo clippy

# Run clippy with all targets and features
cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

### Formatting
```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Format and check
cargo fmt && cargo clippy
```

## Troubleshooting

### Common Build Issues

#### Missing Dependencies
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build
```

#### Compilation Errors
```bash
# Check Rust version
rustc --version

# Update Rust
rustup update

# Check for conflicting versions
rustup show
```

#### Linker Errors
```bash
# Check if required libraries are installed
pkg-config --list-all | grep openssl

# Install missing system dependencies
# (See platform-specific requirements above)
```

### Performance Issues

#### Slow Builds
```bash
# Use release profile for dependencies
cargo build --release

# Use sccache for faster builds
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build
```

#### Memory Issues
```bash
# Build with limited parallelism
cargo build -j 1

# Use swap if needed
# (Platform-specific swap configuration)
```

## Output Locations

### Debug Builds
- **Binary**: `target/debug/jetcrab`
- **Library**: `target/debug/libjetcrab.rlib`

### Release Builds
- **Binary**: `target/release/jetcrab`
- **Library**: `target/release/libjetcrab.rlib`

### Cross-Compilation
- **Output**: `target/{target-triple}/{profile}/jetcrab`

## Next Steps

After building JetCrab:

1. **[Installation Guide](../getting-started/installation.md)** - Install the built binary
2. **[Quick Start Guide](../getting-started/README.md)** - Learn the basics
3. **[JetCrab Guide](../guides/jetcrab-guide.md)** - Comprehensive usage guide
4. **[Contributing Guide](../contributing.md)** - Contribute to development

## Resources

- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Cargo Book**: [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)
- **JetCrab Repository**: [https://github.com/JetCrabCollab/jetcrab](https://github.com/JetCrabCollab/jetcrab)

---

**JetCrab Building** - From source to production 🦀
