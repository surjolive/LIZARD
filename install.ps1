param(
    [string]$ReleaseBaseUrl = $env:LIZARD_RELEASE_BASE_URL
)

$ErrorActionPreference = 'Stop'

if ([string]::IsNullOrWhiteSpace($ReleaseBaseUrl)) {
    throw 'Set LIZARD_RELEASE_BASE_URL to the release download URL before running the installer.'
}

$ReleaseBaseUrl = $ReleaseBaseUrl.TrimEnd('/')
$installDir = Join-Path $env:LOCALAPPDATA 'LIZARD\bin'
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

foreach ($name in @('lz.exe', 'lizard.exe')) {
    $destination = Join-Path $installDir $name
    Invoke-WebRequest -Uri "$ReleaseBaseUrl/$name" -OutFile $destination
}

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$pathEntries = @($userPath -split ';' | Where-Object { $_ })
if ($pathEntries -notcontains $installDir) {
    [Environment]::SetEnvironmentVariable('Path', (($pathEntries + $installDir) -join ';'), 'User')
}

$env:Path = "$installDir;$env:Path"
& (Join-Path $installDir 'lz.exe') --version
Write-Output "LIZARD installed in $installDir"
Write-Output 'Open a new CMD or PowerShell window before using lz.'
