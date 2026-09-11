# LIZARD

<p align="center">
    <img src="img/lizard.png" alt="LIZARD programming language banner" width="420">
</p>

LIZARD is a native Rust-based programming language focused on simple syntax and fast startup.

Current milestone: a native interpreter and CLI with functions, collections,
loops, a growing standard library, terminal animation, and Windows/Linux/macOS
release automation.

<p align="center">
    <a href="https://github.com/surjolive/LIZARD/releases/latest"><img src="https://img.shields.io/github/v/release/surjolive/LIZARD?display_name=tag&sort=semver&style=for-the-badge&color=brightgreen" alt="Latest release"></a>
    <a href="https://github.com/surjolive/LIZARD/actions"><img src="https://img.shields.io/github/actions/workflow/status/surjolive/LIZARD/release.yml?label=release&style=for-the-badge" alt="Release workflow"></a>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/surjolive/LIZARD?style=for-the-badge&color=blue" alt="MIT License"></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/built%20with-Rust-orange?style=for-the-badge&logo=rust" alt="Built with Rust"></a>
</p>

<p align="center">
    <img src="https://img.shields.io/badge/platform-Windows%20x64-0078D6?style=flat-square&logo=windows" alt="Windows x64">
    <img src="https://img.shields.io/badge/platform-Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux">
    <img src="https://img.shields.io/badge/platform-macOS-111111?style=flat-square&logo=apple" alt="macOS">
    <a href="editors/vscode/lizard-vscode"><img src="https://img.shields.io/badge/VS%20Code-extension-007ACC?style=flat-square&logo=visual-studio-code" alt="VS Code extension"></a>
    <a href="docs/LIZARD-GUIDE.md"><img src="https://img.shields.io/badge/docs-LIZARD%20Guide-8A2BE2?style=flat-square" alt="LIZARD Guide"></a>
</p>

<table>
<tr>
<td><strong>Native</strong><br>Rust interpreter and runtime</td>
<td><strong>Readable</strong><br>Simple indentation-based syntax</td>
<td><strong>Practical</strong><br>CLI, REPL, formatter, checker, and builder</td>
<td><strong>Animated</strong><br>Terminal progress animation built in</td>
</tr>
</table>

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

## Project policies

LIZARD is open source under the MIT License. Use the links below for the rules
that govern code, community participation, and security reports.

| Policy | Details |
| --- | --- |
| [MIT License](LICENSE) | Copyright (c) 2026 Surjo Live |
| [Contributing guide](CONTRIBUTING.md) | Development rules, testing, and pull request requirements |
| [Code of Conduct](CODE_OF_CONDUCT.md) | Community standards and expected behavior |
| [Security policy](SECURITY.md) | How to report a security vulnerability privately |

## Usage

| Command | What it does |
| --- | --- |
| `cargo run -- hello.lz` | Run a LIZARD source file |
| `cargo run -- repl` | Start the interactive REPL |
| `cargo run -- -e "say 10 + 20"` | Execute inline source |
| `cargo run -- check hello.lz` | Parse and validate without running |
| `cargo run -- fmt hello.lz` | Print formatted source |
| `cargo run -- fmt --write hello.lz` | Format and save a source file |
| `cargo run -- build hello.lz --output hello.exe` | Build a native output |
| `cargo run -- new MyProject` | Create a project skeleton |
| `cargo run -- doctor` | Diagnose the current installation |
| `cargo run -- update --check` | Check for a newer release safely |
| `cargo run -- upgrade` | Update the installed runtime |
| `cargo run -- --version` | Show version and runtime information |
| `cargo run -- --help` | Show all CLI help |

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

## Features at a glance

### Language and runtime

- **Native Rust implementation** with a fast-starting interpreter and runtime
- **Simple syntax** with readable blocks and `.lz` source files
- **Core values:** numbers, text, booleans, `null`, lists, maps, and functions
- **Variables and expressions:** arithmetic, comparisons, boolean operators, and interpolation
- **Control flow:** `if`, `else`, `while`, `repeat`, and `each`
- **Functions:** parameters, return values, and reusable program logic
- **Collections:** indexing, slicing, list helpers, map values, and iteration

### Standard library

- **Text:** `toText`, `lower`, `upper`, `trim`, `contains`, `split`, and `join`
- **Collections:** `size`, `first`, `last`, `push`, `pop`, `reverse`, and `slice`
- **Numbers:** `toNumber`, `abs`, `floor`, `ceil`, `round`, `min`, `max`, and `clamp`
- **Output:** `say`, `print`, `echo`, `log`, `info`, and `aro`
- **Terminal animation:** `animate(message, frames, delayMs)` for visible progress feedback

### Developer experience

- **REPL** for experimenting with expressions and language features
- **Formatter** with preview and in-place `--write` modes
- **Checker** for validating source without executing it
- **Native builder** for producing a standalone runtime output
- **Project generator** with `new ProjectName`
- **Diagnostics** with `doctor`, version reporting, and clear runtime errors
- **Update tools** with `update`, `upgrade`, and safe `update --check` mode
- **VS Code package** with `.lz` language support, syntax highlighting, snippets,
  run/check commands, and formatting

For the complete installation, CLI, language, build, testing, and editor guide,
see [docs/LIZARD-GUIDE.md](docs/LIZARD-GUIDE.md).

The package ecosystem specification is documented in
[LIZARD_PACKAGE_ECOSYSTEM_SPECIFICATION.md](LIZARD_PACKAGE_ECOSYSTEM_SPECIFICATION.md).
The planned registry is `packages.lizard.dev`; registry and publishing features
are not yet implemented in the current interpreter.

## Animation example

Terminal animation is built into the runtime. The first argument is the message,
the second is the number of frames, and the third is the delay between frames in
milliseconds:

```lz
animate("Loading LIZARD", 8, 120)
print("Ready!")
```

For deterministic scripts and tests, use a delay of `0`:

```lz
animate("Checking", 3, 0)
```

The animation works as a standalone function call, so it does not need to be
wrapped in `say`.

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
| Windows x86_64 | [`lizard.exe`](https://github.com/surjolive/LIZARD/releases/download/v0.1.0/lizard.exe) or [`lz.exe`](https://github.com/surjolive/LIZARD/releases/download/v0.1.0/lz.exe) |
| Linux x86_64 | See the [latest release](https://github.com/surjolive/LIZARD/releases/latest) |
| macOS Intel | See the [latest release](https://github.com/surjolive/LIZARD/releases/latest) |

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
