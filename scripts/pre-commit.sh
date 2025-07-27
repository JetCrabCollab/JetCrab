#!/bin/bash

# Pre-commit script for JetCrab
# This script runs all the same checks that CI runs

set -e

echo "🔍 Running pre-commit checks for JetCrab..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "This script must be run from the project root directory"
    exit 1
fi

# 1. Format check
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    print_error "Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi
print_status "Code formatting is correct"

# 2. Clippy check
echo "🔧 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -W warnings; then
    print_error "Clippy found issues. Please fix them before committing."
    exit 1
fi
print_status "Clippy passed"

# 3. Tests
echo "🧪 Running tests..."
if ! cargo test --all-features; then
    print_error "Tests failed. Please fix them before committing."
    exit 1
fi
print_status "All tests passed"

# 4. Build examples
echo "🏗️  Building examples..."
if ! cargo build --examples --all-features; then
    print_error "Examples failed to build. Please fix them before committing."
    exit 1
fi
print_status "Examples built successfully"

# 5. Documentation check
echo "📚 Checking documentation..."
if ! cargo doc --all-features --no-deps; then
    print_error "Documentation check failed. Please fix doc errors."
    exit 1
fi
print_status "Documentation is correct"

# 6. Security audit
echo "🔒 Running security audit..."
if ! cargo audit; then
    print_warning "Security audit found issues. Please review and update dependencies."
    # Don't fail the script for security issues, just warn
fi
print_status "Security audit completed"

# 7. Check for TODO/FIXME comments
echo "🔍 Checking for TODO/FIXME comments..."
if grep -r "TODO\|FIXME" src/ --include="*.rs"; then
    print_error "Found TODO/FIXME comments. Please address them before committing."
    exit 1
fi
print_status "No TODO/FIXME comments found"

# 8. Check for debug prints
echo "🔍 Checking for debug prints..."
if grep -r "println!\|dbg!" src/ --include="*.rs"; then
    print_error "Found debug prints (println!, dbg!). Please remove them before committing."
    exit 1
fi
print_status "No debug prints found"

# 9. Check binary size (optional)
echo "📏 Checking binary size..."
if cargo build --release 2>/dev/null; then
    BINARY_SIZE=$(stat -c%s target/release/jetcrab 2>/dev/null || stat -f%z target/release/jetcrab 2>/dev/null || echo "0")
    echo "Binary size: $BINARY_SIZE bytes"
    if [ "$BINARY_SIZE" -gt 10485760 ]; then
        print_warning "Binary size exceeds 10MB limit"
    else
        print_status "Binary size is within limits"
    fi
fi

echo ""
print_status "All pre-commit checks passed! 🎉"
echo "You can now commit your changes safely." 