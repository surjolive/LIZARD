# LIZARD

LIZARD is a native Rust-based programming language focused on simple syntax and fast startup.

Current milestone: the first working interpreter for simple statements, variables, arithmetic, and `if` blocks.

## License and project rules

LIZARD is released under the [MIT License](LICENSE), Copyright (c) 2026 Surjo
Live. Contribution standards are defined in [CONTRIBUTING.md](CONTRIBUTING.md).
Community behavior follows [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and
security issues should be reported using [SECURITY.md](SECURITY.md).

## Usage

```
cargo run -- hello.lz
cargo run -- --version
cargo run -- check hello.lz
cargo run -- fmt hello.lz
cargo run -- fmt --write hello.lz
cargo run -- -e "say 10 + 20"
cargo run -- build hello.lz --output hello.exe
cargo run -- doctor
cargo run -- repl
```

The VS Code language package is in `editors/vscode/lizard-vscode`. It registers
`.lz` files, syntax highlighting, snippets, run/check commands, and formatting
through the native LIZARD CLI.

For the complete installation, CLI, language, build, testing, and editor guide,
see [docs/LIZARD-GUIDE.md](docs/LIZARD-GUIDE.md).

## Install on Windows

The installer downloads `lz.exe` and `lizard.exe` into
`%LOCALAPPDATA%\LIZARD\bin` and adds that directory to the user PATH.

From a cloned project directory:

```powershell
$env:LIZARD_RELEASE_BASE_URL = "https://github.com/surjolive/LIZARD/releases/latest/download"
.\install.ps1
```

From Command Prompt:

```bat
set LIZARD_RELEASE_BASE_URL=https://github.com/surjolive/LIZARD/releases/latest/download
install.bat
```

After publishing this repository, direct URL installation works from
PowerShell:

```powershell
$env:LIZARD_RELEASE_BASE_URL = "https://github.com/surjolive/LIZARD/releases/latest/download"
irm https://raw.githubusercontent.com/surjolive/LIZARD/master/install.ps1 | iex
```

From CMD or any shell with `curl` and PowerShell:

```bat
set LIZARD_RELEASE_BASE_URL=https://github.com/surjolive/LIZARD/releases/latest/download
curl.exe -fsSL https://raw.githubusercontent.com/surjolive/LIZARD/master/install.ps1 | powershell.exe -NoProfile -ExecutionPolicy Bypass -Command -
```

The published repository is [surjolive/LIZARD](https://github.com/surjolive/LIZARD).

## Windows release build

Run `build.bat` from Command Prompt or `./build.ps1` from PowerShell. The
release binaries are written to `release/windows-x64/lz.exe` and
`release/windows-x64/lizard.exe`.

The native Windows GNU target requires MinGW. The build scripts validate the
Rust source first and then produce both CLI names from the same runtime.

## Example

```
say "Hello World"

x = 10
y = 20

say x + y

if x < y
    say "LIZARD is simple and fast"
```
