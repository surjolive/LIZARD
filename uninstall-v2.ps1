$ErrorActionPreference = 'Continue'

$installDir = Join-Path $env:LOCALAPPDATA 'LIZARD\bin'

Write-Host "LIZARD Uninstaller"
Write-Host "=================="
Write-Host ""

if (Test-Path $installDir) {
    Write-Host "Removing LIZARD executables from $installDir..."
    Remove-Item -Force -Path $installDir -Recurse -ErrorAction SilentlyContinue
    Write-Host "Successfully removed installation directory"
} else {
    Write-Host "Installation directory not found at $installDir"
}

Write-Host ""

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$pathArray = $userPath -split ';'

if ($pathArray -contains $installDir) {
    $newPathArray = $pathArray | Where-Object { $_ -ne $installDir }
    $newPath = $newPathArray -join ';'
    [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
    Write-Host "Removed LIZARD from PATH"
} else {
    Write-Host "LIZARD not found in PATH"
}

Write-Host ""
Write-Host "LIZARD has been successfully uninstalled."
Write-Host "Please open a new terminal window for changes to take effect."
