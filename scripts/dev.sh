#!/bin/bash

echo "JetCrab Development Script"
echo "========================="

# Format code
echo "Formatting code..."
cargo fmt --all

# Run clippy
echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
echo "Running tests..."
cargo test

# Build
echo "Building project..."
cargo build

echo "Development checks completed!" 