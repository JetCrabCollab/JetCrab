# JetCrab & Claw Installation Guide

This guide covers installation of both JetCrab (JavaScript runtime) and Claw (package manager).

## Quick Install

### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/JetCrabCollab/JetCrab/main/scripts/install.sh | bash
```

### Windows (PowerShell)
```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/JetCrabCollab/JetCrab/main/scripts/install.ps1" | Invoke-Expression
```

## Package Managers

### Homebrew (macOS)
```bash
brew install jetcrab
```

### Chocolatey (Windows)
```powershell
choco install jetcrab
```

### Snap (Linux)
```bash
sudo snap install jetcrab
```

### Scoop (Windows)
```powershell
scoop install jetcrab
```

## Manual Installation

### 1. Download Binary

Visit the [releases page](https://github.com/JetCrabCollab/JetCrab/releases) and download the appropriate binary for your platform:

- **Linux**: `jetcrab-linux-x86_64.tar.gz`
- **Windows**: `jetcrab-windows-x86_64.zip`
- **macOS Intel**: `jetcrab-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `jetcrab-macos-aarch64.tar.gz`

### 2. Extract and Install

#### Linux/macOS
```bash
tar -xzf jetcrab-linux-x86_64.tar.gz
sudo mv jetcrab /usr/local/bin/
sudo chmod +x /usr/local/bin/jetcrab
```

#### Windows
```powershell
Expand-Archive jetcrab-windows-x86_64.zip
Move-Item jetcrab.exe C:\Program Files\JetCrab\
```

### 3. Add to PATH (if needed)

#### Linux/macOS
Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):
```bash
export PATH="/usr/local/bin:$PATH"
```

#### Windows
Add `C:\Program Files\JetCrab` to your system PATH.

## Docker Installation

### Run with Docker
```bash
docker run -it --rm jetcrab/jetcrab:latest
```

### Docker Compose
```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab
docker-compose up
```

## Build from Source

### Prerequisites
- Rust 1.75+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Build Steps
```bash
git clone https://github.com/JetCrabCollab/JetCrab.git
cd JetCrab
cargo build --release
```

## Verify Installation

```bash
jetcrab --version
claw --version
```

Expected output:
```
jetcrab 0.4.0
claw 0.4.0
```

## Troubleshooting

### Permission Denied (Linux/macOS)
```bash
sudo chmod +x /usr/local/bin/jetcrab
```

### Command Not Found
- Ensure the binary is in your PATH
- Restart your terminal
- Check installation directory

### Windows Defender Warning
- Windows Defender may flag the binary as suspicious
- Add an exception for the JetCrab directory
- Or use the package manager installation

## Uninstallation

### Package Managers
```bash
# Homebrew
brew uninstall jetcrab

# Chocolatey
choco uninstall jetcrab

# Snap
sudo snap remove jetcrab
```

### Manual Removal
```bash
# Linux/macOS
sudo rm /usr/local/bin/jetcrab

# Windows
Remove-Item "C:\Program Files\JetCrab\jetcrab.exe"
```

## Support

- **Documentation**: [docs.jetcrab.dev](https://docs.jetcrab.dev)
- **Issues**: [GitHub Issues](https://github.com/JetCrabCollab/JetCrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/JetCrabCollab/JetCrab/discussions)
