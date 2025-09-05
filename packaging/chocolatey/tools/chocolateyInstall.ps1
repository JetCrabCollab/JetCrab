$ErrorActionPreference = 'Stop'

$packageName = 'jetcrab'
$url = 'https://github.com/JetCrabCollab/jetcrab/releases/download/v0.4.0/jetcrab-windows-x86_64.zip'
$checksum = 'placeholder-sha256'
$checksumType = 'sha256'

$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = Join-Path $toolsDir $packageName

# Download and extract
$zipFile = Join-Path $env:TEMP "$packageName.zip"
Get-ChocolateyWebFile -PackageName $packageName -FileFullPath $zipFile -Url $url -Checksum $checksum -ChecksumType $checksumType

# Extract to tools directory
Get-ChocolateyUnzip -FileFullPath $zipFile -Destination $toolsDir

# Create shim
Install-BinFile -Name "jetcrab" -Path (Join-Path $installDir "jetcrab.exe")

# Clean up
Remove-Item $zipFile -Force
