# LIZARD

<p align="center">
    <img src="img/lizard.png" alt="LIZARD programming language banner" width="420">
</p>

LIZARD is a native Rust-based programming language focused on simple syntax and fast startup.

Current milestone: a native interpreter and CLI with functions, collections,
loops, a growing standard library, terminal animation, and Windows/Linux/macOS
release automation.

<table>
<tr>
<td>

## Download latest version

Get the latest LIZARD release from GitHub. The current release is **v0.1.0**.

<a href="https://github.com/surjolive/LIZARD/releases/latest"><strong>Download latest release</strong></a>

<br>

<a href="https://github.com/surjolive/LIZARD/releases/download/v0.1.0/lizard.exe">Windows x64: lizard.exe</a> ·
<a href="https://github.com/surjolive/LIZARD/releases/download/v0.1.0/lz.exe">Windows x64: lz.exe</a>

</td>
</tr>
</table>

## Quick start

LIZARD requires Rust and Cargo. Clone the repository, run the example program,
and verify the compiler with:

```sh
git clone https://github.com/surjolive/LIZARD.git
cd LIZARD
cargo run -- hello.lz
cargo test
```

Use `cargo run -- --help` to see the available CLI commands. The installed
executables are named `lz` and `lizard` and use the same runtime.

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
cargo run -- update --check
```

The VS Code language package is in `editors/vscode/lizard-vscode`. It registers
`.lz` files, syntax highlighting, snippets, run/check commands, and formatting
through the native LIZARD CLI.

## Development

Run the test suite after changing the parser, runtime, or CLI:

```sh
cargo test
```

For a fast compile-only check, use:

```sh
cargo check
```

The full language and editor guide is available at
[docs/LIZARD-GUIDE.md](docs/LIZARD-GUIDE.md).

## Features

- Native Rust parser and runtime
- Variables, arithmetic, booleans, comparisons, and interpolation
- Functions with parameters and return values
- `if`, `else`, `while`, `repeat`, and `each` blocks
- Lists, maps, indexing, slicing, and collection helpers
- Text conversion, searching, splitting, joining, and case helpers
- Numeric helpers including `abs`, `min`, `max`, and `clamp`
- Terminal progress animation with `animate(message, frames, delayMs)`
- REPL, formatter, checker, project creation, and native build commands
- `update` and `upgrade` commands with safe `--check` mode
- Release downloads for Windows, Linux, and macOS

For the complete installation, CLI, language, build, testing, and editor guide,
see [docs/LIZARD-GUIDE.md](docs/LIZARD-GUIDE.md).

The package ecosystem specification is documented in
[LIZARD_PACKAGE_ECOSYSTEM_SPECIFICATION.md](LIZARD_PACKAGE_ECOSYSTEM_SPECIFICATION.md).
The planned registry is `packages.lizard.dev`; registry and publishing features
are not yet implemented in the current interpreter.

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

## Download for Windows, Linux, and macOS

Download the latest native release from the
[GitHub Releases page](https://github.com/surjolive/LIZARD/releases/latest):

| Platform | Download |
| --- | --- |
| Windows x86_64 | `lizard-windows-x86_64.zip` |
| Linux x86_64 | `lizard-linux-x86_64.tar.gz` |
| macOS Intel | `lizard-macos-x86_64.tar.gz` |

On Linux or macOS, install the latest release with one command:

```sh
curl -fsSL https://raw.githubusercontent.com/surjolive/LIZARD/master/install.sh | sh
```

The Unix installer places `lz` and `lizard` in `~/.local/bin`. Set
`LIZARD_INSTALL_DIR` to choose another location. Releases are built
automatically by GitHub Actions whenever a `v*` tag is pushed.

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
