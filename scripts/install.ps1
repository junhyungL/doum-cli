# Doum CLI Installation Script for Windows
# Run this script with: iwr -useb https://raw.githubusercontent.com/junhyungL/doum-cli/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# Configuration
$Repo = "junhyungL/doum-cli"
$BinaryName = "doum.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\doum-cli"

# Colors
function Write-ColorOutput($ForegroundColor, $Message) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    Write-Host $Message
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Success($Message) { Write-ColorOutput Green $Message }
function Write-Info($Message) { Write-ColorOutput Cyan $Message }
function Write-Warning($Message) { Write-ColorOutput Yellow $Message }
function Write-Error($Message) { Write-ColorOutput Red $Message }

# Detect architecture
function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }
}

# Get latest release version
function Get-LatestVersion {
    Write-Info "Fetching latest version..."
    
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $version = $response.tag_name
        
        if ([string]::IsNullOrEmpty($version)) {
            throw "Failed to get version"
        }
        
        Write-Success "Latest version: $version"
        return $version
    }
    catch {
        Write-Error "Failed to fetch latest version: $_"
        exit 1
    }
}

# Download and install
function Install-Doum {
    param (
        [string]$Version,
        [string]$Arch
    )
    
    $assetName = "doum-windows-$Arch"
    $downloadUrl = "https://github.com/$Repo/releases/download/$Version/$assetName.zip"
    $tmpDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
    New-Item -ItemType Directory -Path $tmpDir | Out-Null
    
    $zipFile = Join-Path $tmpDir "$assetName.zip"
    
    Write-Info "Downloading $assetName..."
    Write-Info "URL: $downloadUrl"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipFile
    }
    catch {
        Write-Error "Failed to download: $_"
        Write-Error "URL was: $downloadUrl"
        Remove-Item -Recurse -Force $tmpDir
        exit 1
    }
    
    Write-Info "Extracting..."
    Expand-Archive -Path $zipFile -DestinationPath $tmpDir -Force
    
    # Create install directory
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }
    
    Write-Info "Installing to $InstallDir..."
    $source = Join-Path $tmpDir $BinaryName
    $destination = Join-Path $InstallDir $BinaryName
    
    # Remove old version if exists
    if (Test-Path $destination) {
        Remove-Item $destination -Force
    }
    
    Move-Item -Path $source -Destination $destination -Force
    
    # Cleanup
    Remove-Item -Recurse -Force $tmpDir
    
    Write-Success "Doum CLI installed successfully!"
}

# Add to PATH
function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Info "Adding $InstallDir to PATH..."
        
        $newPath = "$currentPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        
        # Update current session
        $env:Path = "$env:Path;$InstallDir"
        
        Write-Success "Added to PATH"
        Write-Warning "You may need to restart your terminal for PATH changes to take effect."
    }
    else {
        Write-Info "$InstallDir is already in PATH"
    }
}

# Verify installation
function Test-Installation {
    $doumPath = Join-Path $InstallDir $BinaryName
    
    if (Test-Path $doumPath) {
        Write-Success "`nVerification successful!"
        Write-Info "Run 'doum --help' to get started."
        Write-Warning "Note: You may need to restart your terminal if 'doum' command is not found."
    }
    else {
        Write-Error "`nInstallation verification failed!"
        exit 1
    }
}

# Main
function Main {
    Write-Output ""
    Write-Output "====================================="
    Write-Output "   Doum CLI Installation Script"
    Write-Output "====================================="
    Write-Output ""
    
    $arch = Get-Architecture
    Write-Info "Architecture: $arch"
    Write-Output ""
    
    $version = Get-LatestVersion
    Write-Output ""

    Install-Doum -Version $version -Arch $arch
    Write-Output ""

    Add-ToPath
    Write-Output ""

    Test-Installation
    Write-Output ""

    Write-Success "Done!"
}

Main
