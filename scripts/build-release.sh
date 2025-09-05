#!/bin/bash
set -e

# Build script for JetCrab releases
# Usage: ./scripts/build-release.sh [version]

VERSION=${1:-"0.4.0"}
TARGET_DIR="target/release"
DIST_DIR="dist"

echo "🦀 Building JetCrab v$VERSION"

# Clean previous builds
rm -rf $DIST_DIR
mkdir -p $DIST_DIR

# Build for current platform
echo "📦 Building for current platform..."
cargo build --release

# Copy binaries
cp $TARGET_DIR/jetcrab $DIST_DIR/
cp $TARGET_DIR/claw $DIST_DIR/

# Create platform-specific package
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 Creating Linux package..."
    tar -czf $DIST_DIR/jetcrab-linux-$(uname -m).tar.gz -C $DIST_DIR jetcrab claw
    echo "✅ Linux package created: $DIST_DIR/jetcrab-linux-$(uname -m).tar.gz"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 Creating macOS package..."
    tar -czf $DIST_DIR/jetcrab-macos-$(uname -m).tar.gz -C $DIST_DIR jetcrab claw
    echo "✅ macOS package created: $DIST_DIR/jetcrab-macos-$(uname -m).tar.gz"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "🪟 Creating Windows package..."
    cd $DIST_DIR
    zip -r jetcrab-windows-$(uname -m).zip jetcrab.exe claw.exe
    cd ..
    echo "✅ Windows package created: $DIST_DIR/jetcrab-windows-$(uname -m).zip"
fi

# Generate checksums
echo "🔐 Generating checksums..."
cd $DIST_DIR
for file in jetcrab-*; do
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        certutil -hashfile "$file" SHA256 > "$file.sha256"
    else
        sha256sum "$file" > "$file.sha256"
    fi
done
cd ..

echo "🎉 Build complete! Packages available in $DIST_DIR/"
ls -la $DIST_DIR/
