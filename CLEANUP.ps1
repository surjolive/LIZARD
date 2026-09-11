#!/usr/bin/env powershell
<#
LIZARD Project Cleanup Script
Removes all unused files and folders
#>

Write-Host "======================================" -ForegroundColor Cyan
Write-Host "LIZARD Project Cleanup" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

$baseDir = "C:\Users\SURJO LIVE\Downloads\LIZARD"
$deletedCount = 0
$skippedCount = 0

# Function to safely delete files
function Remove-ItemSafe {
    param([string]$Path, [string]$DisplayName)
    
    if (Test-Path $Path) {
        try {
            Remove-Item $Path -Recurse -Force -ErrorAction Stop
            Write-Host "✓ Deleted: $DisplayName" -ForegroundColor Green
            $script:deletedCount++
        } catch {
            Write-Host "✗ Failed to delete: $DisplayName" -ForegroundColor Red
            $script:skippedCount++
        }
    } else {
        Write-Host "- Skipped: $DisplayName (not found)" -ForegroundColor Yellow
        $script:skippedCount++
    }
}

Write-Host "Cleaning up unused files..." -ForegroundColor Yellow
Write-Host ""

# Remove old uninstall script version
Remove-ItemSafe "$baseDir\uninstall-v2.ps1" "Old uninstall script (uninstall-v2.ps1)"

# Remove build helper scripts
Remove-ItemSafe "$baseDir\build.bat" "Build helper (build.bat)"
Remove-ItemSafe "$baseDir\build.ps1" "Build helper (build.ps1)"

# Remove Cargo.lock (should be regenerated)
Remove-ItemSafe "$baseDir\Cargo.lock" "Cargo lock file (Cargo.lock)"

# Remove CI/CD configuration
Remove-ItemSafe "$baseDir\.github" "GitHub workflows (.github folder)"

# Remove old release binaries and checksums
Remove-ItemSafe "$baseDir\release\windows-x64\hello.exe" "Old test binary (hello.exe)"
Remove-ItemSafe "$baseDir\release\windows-x64\hello.lz" "Old test file (hello.lz in release)"
Remove-ItemSafe "$baseDir\release\windows-x64\lz.exe.sha256" "Old checksum (lz.exe.sha256)"
Remove-ItemSafe "$baseDir\release\windows-x64\lizard.exe.sha256" "Old checksum (lizard.exe.sha256)"

# Remove test files from root (optional - keep for testing)
# Remove-ItemSafe "$baseDir\test_all.lz" "Test program (test_all.lz)"
# Remove-ItemSafe "$baseDir\test_all_commands.ps1" "Test script (test_all_commands.ps1)"

# Remove image folder if not needed (optional)
# Remove-ItemSafe "$baseDir\img" "Images folder (img)"

# Clean up any .d files (dependency files)
Get-ChildItem "$baseDir" -Filter "*.d" -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
    try {
        Remove-Item $_.FullName -Force -ErrorAction Stop
        Write-Host "✓ Deleted: $($_.Name)" -ForegroundColor Green
        $script:deletedCount++
    } catch {
        $script:skippedCount++
    }
}

Write-Host ""
Write-Host "======================================" -ForegroundColor Cyan
Write-Host "Cleanup Summary" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host "Files deleted: $deletedCount" -ForegroundColor Green
Write-Host "Files skipped: $skippedCount" -ForegroundColor Yellow
Write-Host ""

Write-Host "Remaining files in project:" -ForegroundColor Cyan
Write-Host ""

# Show project structure
$structure = @{
    "Core Files" = @(
        "src/main.rs",
        "Cargo.toml",
        ".cargo/config.toml"
    )
    "Documentation" = @(
        "README.md",
        "LICENSE",
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "SECURITY.md",
        "INSTALL-WINDOWS.md",
        "LIZARD-GUIDE.md",
        "IMPLEMENTATION_REPORT.md"
    )
    "Installation Scripts" = @(
        "install.bat",
        "install.ps1",
        "install.sh",
        "uninstall.bat",
        "uninstall.ps1",
        "uninstall.sh"
    )
    "Precompiled Binaries" = @(
        "release/windows-x64/lz.exe",
        "release/windows-x64/lizard.exe"
    )
    "Source Code" = @(
        "src/main.rs (2100+ lines)",
        "Complete LIZARD language implementation"
    )
    "Tests & Examples" = @(
        "tests/",
        "hello.lz",
        "test_all.lz",
        "test_all_commands.ps1"
    )
    "Editor Support" = @(
        "editors/vscode/ (Syntax highlighting)",
        "editors/vscode/lizard-vscode/package.json"
    )
}

foreach ($category in $structure.Keys) {
    Write-Host "$category:" -ForegroundColor Cyan
    $structure[$category] | ForEach-Object {
        Write-Host "  ✓ $_"
    }
    Write-Host ""
}

Write-Host "======================================" -ForegroundColor Green
Write-Host "Cleanup Complete!" -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Green
