#!/bin/bash
set -e

# Quick install script for JetCrab
# Usage: curl -sSL https://raw.githubusercontent.com/JetCrabCollab/jetcrab/main/scripts/install.sh | bash

REPO="JetCrabCollab/jetcrab"
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "🦀 Installing JetCrab..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    armv7l)
        ARCH="armv7"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS names
case $OS in
    linux)
        OS="linux"
        EXT="tar.gz"
        ;;
    darwin)
        OS="macos"
        EXT="tar.gz"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release info
echo "📡 Fetching latest release info..."
RELEASE_INFO=$(curl -s $LATEST_RELEASE_URL)
VERSION=$(echo $RELEASE_INFO | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$')
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/jetcrab-$OS-$ARCH.$EXT"

echo "📦 Downloading JetCrab $VERSION for $OS-$ARCH..."

TEMP_DIR=$(mktemp -d)
cd $TEMP_DIR

curl -L -o "jetcrab.$EXT" "$DOWNLOAD_URL"

if [[ "$EXT" == "tar.gz" ]]; then
    tar -xzf "jetcrab.$EXT"
else
    unzip -o "jetcrab.$EXT"
fi

# Install to /usr/local/bin
sudo mv jetcrab /usr/local/bin/
sudo chmod +x /usr/local/bin/jetcrab

# Cleanup
cd /
rm -rf $TEMP_DIR

echo "✅ JetCrab installed successfully!"
echo "🚀 Run 'jetcrab --version' to verify installation"
echo "💡 Install CPM separately: cargo install cpm"
