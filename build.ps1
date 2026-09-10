$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (-not (Test-Path $cargo)) { $cargo = 'cargo' }
$mingw = 'C:\mingw64\bin'
if (Test-Path $mingw) { $env:Path = "$mingw;$env:Path" }
& $cargo fmt -- --check
& $cargo check
& $cargo build --release --target x86_64-pc-windows-gnu
$release = Join-Path $root 'release\windows-x64'
New-Item -ItemType Directory -Force -Path $release | Out-Null
Copy-Item (Join-Path $root 'target\x86_64-pc-windows-gnu\release\lizard.exe') (Join-Path $release 'lizard.exe') -Force
Copy-Item (Join-Path $root 'target\x86_64-pc-windows-gnu\release\lz.exe') (Join-Path $release 'lz.exe') -Force
Get-FileHash (Join-Path $release 'lz.exe') -Algorithm SHA256 | Set-Content (Join-Path $release 'lz.exe.sha256')
Get-FileHash (Join-Path $release 'lizard.exe') -Algorithm SHA256 | Set-Content (Join-Path $release 'lizard.exe.sha256')
Write-Output 'BUILD SUCCESSFUL'
Write-Output "LIZARD CLI: $release\lz.exe"
Write-Output "LIZARD: $release\lizard.exe"
