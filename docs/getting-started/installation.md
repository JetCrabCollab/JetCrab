# JetCrab Installation Guide

## Overview

This guide will help you set up JetCrab on your system for development and usage.

## Prerequisites

### **System Requirements**
- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- **Architecture**: x86_64 or ARM64
- **Memory**: Minimum 4GB RAM, 8GB recommended
- **Storage**: 2GB free space for development

### **Required Software**
- **Rust**: Version 1.70 or higher
- **Cargo**: Rust's package manager (included with Rust)
- **Git**: Version 2.0 or higher
- **Build Tools**: Platform-specific build dependencies

## Installation Steps

### **Step 1: Install Rust**

#### **Windows**
```bash
# Download and run rustup-init.exe from https://rustup.rs/
# Or use winget:
winget install Rust.Rust
```

#### **macOS**
```bash
# Using Homebrew
brew install rust

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### **Linux**
```bash
# Ubuntu/Debian
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or using package manager
sudo apt update
sudo apt install rustc cargo
```

### **Step 2: Verify Installation**

```bash
# Check Rust version
rustc --version
cargo --version

# Expected output:
# rustc 1.70.0 (90c541806 2023-05-31)
# cargo 1.70.0 (ec8a8a0ca 2023-05-26)
```

### **Step 3: Clone JetCrab Repository**

```bash
# Clone the repository
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab

# Verify the clone
ls -la
```

### **Step 4: Build JetCrab**

```bash
# Build the project
cargo build

# For release build (optimized)
cargo build --release

# Expected output: Compilation successful
```

### **Step 5: Test Installation**

```bash
# Run the basic example
cargo run --example basic_usage

# Expected output: JavaScript examples running successfully
```

## Development Setup

### **Install Development Dependencies**

```bash
# Install additional tools for development
cargo install cargo-watch  # For file watching
cargo install cargo-audit   # For security auditing
cargo install cargo-tarpaulin  # For code coverage
```

### **IDE Setup**

#### **VS Code**
1. Install the "Rust" extension
2. Install "rust-analyzer" extension
3. Open the JetCrab folder
4. Enable auto-formatting on save

#### **IntelliJ IDEA / CLion**
1. Install the Rust plugin
2. Open the JetCrab project
3. Configure Rust toolchain
4. Enable auto-imports

### **Configure Git Hooks**

```bash
# Install pre-commit hooks
./scripts/pre-commit.sh

# Or manually set up hooks
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Verification

### **Run Tests**
```bash
# Run all tests
cargo test

# Note: Some tests may currently fail due to ongoing development
```

### **Run Examples**
```bash
# Basic usage example
cargo run --example basic_usage

# Other examples
cargo run --example vm_demo
cargo run --example simple_test
```

### **Check Code Quality**
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security issues
cargo audit
```

## Troubleshooting

### **Common Issues**

#### **Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build

# Update Rust toolchain
rustup update
```

#### **Dependency Issues**
```bash
# Update dependencies
cargo update

# Check for outdated packages
cargo outdated
```

#### **Permission Issues (Linux/macOS)**
```bash
# Fix permissions
chmod +x scripts/*.sh
chmod +x .git/hooks/*
```

### **Platform-Specific Issues**

#### **Windows**
- Ensure Visual Studio Build Tools are installed
- Use PowerShell or Command Prompt
- Check Windows Defender exclusions

#### **macOS**
- Install Xcode Command Line Tools
- Use Homebrew for dependencies
- Check Gatekeeper settings

#### **Linux**
- Install build essentials
- Check system libraries
- Verify Python installation

## Configuration

### **Environment Variables**

```bash
# Set development environment
export RUST_LOG=debug
export RUST_BACKTRACE=1

# For Windows PowerShell
$env:RUST_LOG="debug"
$env:RUST_BACKTRACE="1"
```

### **Cargo Configuration**

Create `~/.cargo/config.toml`:
```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[profile.dev]
opt-level = 1
debug = true

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## Next Steps

### **For Users**
1. Read the **[Project README](../README.md)**: Project overview
2. Try the **[Basic Examples](../examples/)**: Working examples
3. Check **[Implementation Status](../implementation/)**: Current features

### **For Contributors**
1. Read the **[CONTRIBUTING.md](../CONTRIBUTING.md)** guide
2. Review **[Architecture Documentation](../architecture/)**: System design
3. Check **[Implementation Status](../implementation/)**: Current work

### **For Developers**
1. Explore the **[Source Code](../src/)**: Codebase structure
2. Review **[Module Architecture](../architecture/module-architecture.md)**: Module organization
3. Check **[Test Suite](../tests/)**: Testing framework

## Support

### **Getting Help**
- **Documentation**: Check this guide and related docs
- **Issues**: Report problems on GitHub
- **Discussions**: Join community discussions
- **Code Review**: Submit pull requests

### **Resources**
- **[Rust Book](https://doc.rust-lang.org/book/)**: Rust language guide
- **[Cargo Book](https://doc.rust-lang.org/cargo/)**: Package manager guide
- **[JetCrab Repository](https://github.com/JetCrabCollab/JetCrab)**: Source code

---

**Note**: This installation guide covers the basic setup for JetCrab. For advanced configuration and development setup, refer to the architecture and implementation documentation. 