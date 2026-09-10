# Windows installation

Build native binaries with `build.bat` or `build.ps1`. The release files are written to `release/windows-x64`.

## Install from a release URL

Set the release URL containing `lz.exe` and `lizard.exe`, then run the
installer. It installs both binaries to `%LOCALAPPDATA%\LIZARD\bin` and adds
that directory to the user PATH.

PowerShell:

```powershell
$env:LIZARD_RELEASE_BASE_URL = "https://github.com/surjolive/LIZARD/releases/latest/download"
.\install.ps1
```

Command Prompt:

```bat
set LIZARD_RELEASE_BASE_URL=https://github.com/surjolive/LIZARD/releases/latest/download
install.bat
```

Direct URL form after the repository is published:

```powershell
$env:LIZARD_RELEASE_BASE_URL = "https://github.com/surjolive/LIZARD/releases/latest/download"
irm https://raw.githubusercontent.com/surjolive/LIZARD/master/install.ps1 | iex
```

The published repository is [surjolive/LIZARD](https://github.com/surjolive/LIZARD).

Linux and macOS users can install the latest x86_64 release with:

```sh
curl -fsSL https://raw.githubusercontent.com/surjolive/LIZARD/master/install.sh | sh
```

For a per-user installation, add that directory to the user PATH, then open a new PowerShell or CMD window:

```powershell
$bin = (Resolve-Path .\release\windows-x64).Path
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ";$bin", 'User')
```

Verify with:

```text
lz --version
lz --help
lz doctor
```

The build does not create fake launchers: `lz.exe` and `lizard.exe` are native Rust binaries built from the same runtime.
