$ErrorActionPreference = 'Continue'

$installDir = Join-Path $env:LOCALAPPDATA 'LIZARD\bin'

Write-Output "LIZARD Uninstaller"
Write-Output "=================="
Write-Output ""

if (Test-Path $installDir) {
    Write-Output "Removing LIZARD executables from $installDir..."
    Remove-Item -Force -Path $installDir -Recurse -ErrorAction SilentlyContinue
    Write-Output "✓ Removed installation directory"
} else {
    Write-Output "⚠ Installation directory not found at $installDir"
}

Write-Output ""

# Remove from PATH
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$pathArray = $userPath -split ';'

if ($pathArray -contains $installDir) {
    $newPathArray = $pathArray | Where-Object { $_ -ne $installDir }
    $newPath = $newPathArray -join ';'
    [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    Write-Output "✓ Removed LIZARD from PATH"
} else {
    Write-Output "⚠ LIZARD not found in PATH"
}

Write-Output ""
Write-Output "LIZARD has been successfully uninstalled."
Write-Output "Please open a new terminal window for changes to take effect."

Write-Output "`nLIZARD has been successfully uninstalled."
Write-Output "Please open a new terminal window for changes to take effect."
