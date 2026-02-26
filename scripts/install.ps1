# Quick install script for JetCrab (Windows)
# Usage: Invoke-WebRequest -Uri "https://raw.githubusercontent.com/JetCrabCollab/jetcrab/main/scripts/install.ps1" | Invoke-Expression

$ErrorActionPreference = "Stop"

$Repo = "JetCrabCollab/jetcrab"
$LatestReleaseUrl = "https://api.github.com/repos/$Repo/releases/latest"

Write-Host "🦀 Installing JetCrab..." -ForegroundColor Cyan

# Detect architecture
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$OS = "windows"
$Ext = "zip"

Write-Host "📡 Fetching latest release info..." -ForegroundColor Yellow

# Get latest release info
$ReleaseInfo = Invoke-RestMethod -Uri $LatestReleaseUrl
$Version = $ReleaseInfo.tag_name
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/jetcrab-$OS-$Arch.$Ext"

Write-Host "📦 Downloading JetCrab $Version for $OS-$Arch..." -ForegroundColor Yellow

# Download and install
$TempDir = [System.IO.Path]::GetTempPath()
$ZipFile = Join-Path $TempDir "jetcrab.zip"
$ExtractDir = Join-Path $TempDir "jetcrab"

Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipFile

# Extract
Expand-Archive -Path $ZipFile -DestinationPath $ExtractDir -Force

# Install to Program Files
$InstallDir = "C:\Program Files\JetCrab"
$BinDir = Join-Path $InstallDir "bin"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force
}

if (-not (Test-Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir -Force
}

Copy-Item -Path (Join-Path $ExtractDir "jetcrab.exe") -Destination $BinDir -Force

# Add to PATH
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($CurrentPath -notlike "*$BinDir*") {
    $NewPath = $CurrentPath + ";" + $BinDir
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "Machine")
    $env:PATH = $env:PATH + ";" + $BinDir
}

# Cleanup
Remove-Item $ZipFile -Force
Remove-Item $ExtractDir -Recurse -Force

Write-Host "✅ JetCrab installed successfully!" -ForegroundColor Green
Write-Host "🚀 Run 'jetcrab --version' to verify installation" -ForegroundColor Green
Write-Host "💡 Install CPM separately: cargo install cpm" -ForegroundColor Cyan
Write-Host "⚠️  You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
