# JetCrab Installation Guide

## Overview

This guide will help you install and set up JetCrab v0.4.0 on your system.

## Prerequisites

### System Requirements
- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- **Architecture**: x86_64 or ARM64
- **Memory**: Minimum 4GB RAM, 8GB recommended
- **Storage**: 2GB free space for development

### Required Software
- **Rust**: Version 1.70 or higher
- **Cargo**: Rust's package manager (included with Rust)
- **Git**: Version 2.0 or higher

## Installation Methods

### Method 1: Build from Source (Recommended)

#### Step 1: Install Rust
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal or run:
source ~/.cargo/env
```

#### Step 2: Clone and Build JetCrab
```bash
# Clone the repository
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab

# Build in release mode
cargo build --release

# The binary will be available at:
# target/release/jetcrab.exe (Windows)
# target/release/jetcrab (Unix-like systems)
```

### Method 2: Install via Cargo (When Available)
```bash
# Install JetCrab globally
cargo install jetcrab

# Verify installation
jetcrab --version
```

### Method 3: Download Pre-built Binaries (When Available)
```bash
# Download from GitHub Releases
# https://github.com/JetCrabCollab/JetCrab/releases

# Extract and add to PATH
# Windows: Add to system PATH
# Unix-like: Copy to /usr/local/bin or add to PATH
```

## Verification

### Test Installation
```bash
# Check version
jetcrab --version

# Run a simple test
jetcrab eval "console.log('Hello, JetCrab!'); 42 + 8"

# Expected output:
# Hello, JetCrab!
# 50
```

### Test Package Manager
```bash
# Check Claw version
claw --version

# Initialize a test project
claw init test-project
cd test-project

# Install a package
claw install lodash
```

## Development Setup

### For Contributors
```bash
# Clone the repository
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab

# Install development dependencies
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open
```

### IDE Setup
#### Visual Studio Code
```bash
# Install Rust extension
code --install-extension rust-lang.rust-analyzer

# Install additional extensions
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

#### IntelliJ IDEA / CLion
```bash
# Install Rust plugin
# File -> Settings -> Plugins -> Search "Rust"
```

## Troubleshooting

### Common Issues

#### Build Fails
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### Permission Denied (Unix-like systems)
```bash
# Make binary executable
chmod +x target/release/jetcrab

# Or install to system directory
sudo cp target/release/jetcrab /usr/local/bin/
```

#### Windows Build Issues
```bash
# Install Visual Studio Build Tools
# Or install Visual Studio Community with C++ workload

# Update Rust toolchain
rustup update stable-x86_64-pc-windows-msvc
```

### Getting Help
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/JetCrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/JetCrab/discussions)
- **Documentation**: [docs/](../README.md)

## Next Steps

After successful installation:

1. **Read the [Quick Start Guide](README.md)**
2. **Explore [Examples](../examples/)**
3. **Check [Architecture Documentation](../architecture/engine-overview.md)**
4. **Join the [Community](https://github.com/JetCrabCollab/JetCrab/discussions)**

---

**JetCrab v0.4.0** - Modern JavaScript Runtime in Rust