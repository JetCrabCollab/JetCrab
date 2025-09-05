#!/bin/bash
set -e

# Script to create a new JetCrab release
# Usage: ./scripts/create-release.sh [version] [message]

VERSION=${1:-"0.4.0"}
MESSAGE=${2:-"Release v$VERSION"}

echo "🚀 Creating release v$VERSION..."

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
sed -i.bak "s/version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in package files
echo "📝 Updating version in package files..."
sed -i.bak "s/version: '.*'/version: '$VERSION'/" packaging/snapcraft.yaml
rm packaging/snapcraft.yaml.bak

sed -i.bak "s/version: '.*'/version: '$VERSION'/" packaging/appimage.yml
rm packaging/appimage.yml.bak

# Update Homebrew formula
echo "📝 Updating Homebrew formula..."
sed -i.bak "s/version \".*\"/version \"$VERSION\"/" packaging/jetcrab.rb
rm packaging/jetcrab.rb.bak

# Update Chocolatey package
echo "📝 Updating Chocolatey package..."
sed -i.bak "s/<version>.*<\/version>/<version>$VERSION<\/version>/" packaging/jetcrab.nuspec
rm packaging/jetcrab.nuspec.bak

# Build release
echo "🔨 Building release..."
./scripts/build-release.sh $VERSION

# Create git tag
echo "🏷️  Creating git tag..."
git add .
git commit -m "Release v$VERSION" || true
git tag -a "v$VERSION" -m "$MESSAGE"

echo "✅ Release v$VERSION created successfully!"
echo "📦 Packages available in dist/"
echo "🏷️  Git tag created: v$VERSION"
echo ""
echo "Next steps:"
echo "1. Push the tag: git push origin v$VERSION"
echo "2. Push changes: git push origin main"
echo "3. GitHub Actions will automatically create the release"
echo "4. Update package managers with new version"


