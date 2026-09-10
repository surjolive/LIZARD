# LIZARD Complete Guide

LIZARD is a native Rust-based programming language. Source files use the `.lz` extension. The project provides two CLI names, `lz` and `lizard`, from the same runtime.

Current language version: `0.1.0`

## 1. Project Layout

```text
LIZARD/
├── src/main.rs                         # Parser, evaluator, CLI, and runtime
├── tests/                              # Native integration tests
├── hello.lz                            # Example program
├── editors/vscode/lizard-vscode/       # VS Code language package
├── build.bat                           # Windows CMD release build
├── build.ps1                           # Windows PowerShell release build
├── INSTALL-WINDOWS.md                  # PATH installation notes
└── docs/LIZARD-GUIDE.md                 # This guide
```

## 2. Requirements

Development requires:

- Rust and Cargo
- Windows x64 for the Windows executable workflow
- A native linker

For the GNU Windows target, install MinGW/WinLibs and ensure this command works:

```powershell
x86_64-w64-mingw32-gcc --version
```

For the MSVC target, install Visual Studio Build Tools with the C++ workload so `link.exe` is available.

The language runtime itself is native Rust. It does not use Python, Node, or a Python wrapper.

## 3. Run From Source

Open PowerShell in the repository root:

```powershell
cd "C:\Users\SURJO LIVE\Downloads\LIZARD"
```

Run the example:

```powershell
cargo run -- hello.lz
```

Direct execution preserves the same behavior:

```powershell
cargo run -- run hello.lz
```

A source file can also be passed with a relative or absolute path:

```powershell
cargo run -- .\hello.lz
cargo run -- "C:\Projects\Test\hello.lz"
```

## 4. CLI Commands

The installed executable supports these commands:

```text
lz                         Start the REPL
lz hello.lz                Run a source file
lz run hello.lz            Run a source file explicitly
lz build hello.lz          Build a native runtime copy
lz check hello.lz          Parse/check without execution
lz fmt hello.lz            Print formatted source
lz fmt --write hello.lz   Format and overwrite the source file
lz test                    Show the development test command
lz repl                    Start the REPL
lz new MyProject           Create a project skeleton
lz doctor                 Diagnose the current installation
lz -e "say 10 + 20"        Execute inline LIZARD source
lz --version              Print version and runtime information
lz --help                 Print command help
```

`lizard` accepts the same commands as `lz`.

## 5. Version and Help

```powershell
lz --version
lz --help
```

Version information comes from the Cargo package version, so the Rust package and CLI use one version source.

## 6. REPL

Start the REPL with either command:

```powershell
lz
lz repl
```

Example session:

```text
LIZARD 0.1.0
Runtime: Native
> say "Hello"
Hello
> say 10 + 20
30
> name = "Surjo"
> say name
Surjo
> exit
```

Use `exit` or `quit` to leave the REPL.

## 7. Language Syntax

### Output and variables

```lz
say "Hello World"
x = 10
y = 20
say x + y
```

### Conditions

```lz
if x < y
    say "x is smaller"
else
    say "x is not smaller"
end
```

### Functions

```lz
fn add(a, b)
    return a + b
end

say add(10, 20)
```

### Repeat loops

```lz
repeat 3
    say "again"
end
```

### Lists and maps

```lz
numbers = [10, 20, 30]
items = {"name": "Surjo", "age": 20}

say numbers[1]
say items["name"]

each number in numbers
    say number
end
```

Supported core values include numbers, text, booleans, null, lists, maps, and functions.

## 8. Native Built-ins

The runtime provides these native functions:

```lz
say size([10, 20, 30])       # 3
say typeOf(10)               # number
say toText(42)               # 42
say toNumber("12.5")         # 12.5
say abs(-4)                  # 4
say floor(4.8)               # 4
say ceil(4.2)                # 5
say round(4.6)               # 5
say min(8, 3, 6)             # 3
say max(8, 3, 6)             # 8
numbers = push([1, 2], 3)
say pop(numbers)              # 3
say reverse("lizard")         # drazil
say slice("lizard", 1, 4)     # iza
say clamp(15, 0, 10)           # 10
```

`size` accepts text, lists, and maps. `toNumber` accepts a number or numeric
text. Numeric functions report a clear runtime error when passed the wrong
value type or argument count.

`push` returns a new list with one value appended, while `pop` reads the last
value without changing the original list. `reverse` supports text and lists.
`slice` uses an inclusive start and exclusive end index.

### Terminal animation

```lz
animate("Loading", 5, 100)
```

`animate` displays a terminal progress animation with a message, frame count,
and delay in milliseconds. Use a delay of `0` for scripts and tests that need
deterministic, immediate output.

### Friendly output

```lz
aro("hello")
print("hello")
echo("hello")
log("hello")
info("hello")
```

`aro`, `print`, `echo`, `log`, and `info` print a value and return it.

## 9. Checking and Formatting

Check source without running it:

```powershell
lz check hello.lz
```

Print canonical formatting:

```powershell
lz fmt hello.lz
```

Format the file in place:

```powershell
lz fmt --write hello.lz
```

The formatter uses four spaces for block indentation.

## 10. Native Build

Build the Rust CLI in release mode with the PowerShell script:

```powershell
.\build.ps1
```

Or from Command Prompt:

```cmd
build.bat
```

Expected output files:

```text
release/windows-x64/lz.exe
release/windows-x64/lizard.exe
```

Both binaries are native Rust executables built from the same source and runtime.

### Build a LIZARD program

```powershell
lz build hello.lz
```

This validates the source, copies the native LIZARD runtime to `hello.exe`, and places the source beside it as `hello.lz`. The generated executable runs the real LIZARD parser/evaluator and does not invoke Python.

Custom output path:

```powershell
lz build hello.lz --output myapp.exe
```

Run the generated application from its output directory:

```powershell
.\hello.exe
```

The generated executable uses its adjacent `.lz` file when launched without arguments. Keep that source file beside the executable.

## 11. Install On Windows

After a successful release build, add the release directory to the user PATH:

```powershell
$bin = (Resolve-Path .\release\windows-x64).Path
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
[Environment]::SetEnvironmentVariable('Path', "$userPath;$bin", 'User')
```

Open a new PowerShell or CMD window, then verify:

```powershell
where.exe lz
where.exe lizard
lz --version
lz doctor
```

`where.exe lz` should point to `release/windows-x64/lz.exe` or the installation directory selected by you.

## 12. Project Creation

Create a project skeleton:

```powershell
lz new MyProject
cd MyProject
lz run
```

The generated project contains:

```text
MyProject/
├── lizard.json
├── src/main.lz
├── tests/
└── README.md
```

`lizard.json` currently uses `src/main.lz` as the project entry point.

## 13. Doctor

Run:

```powershell
lz doctor
```

The command reports the current executable, native runtime, version, and operating system. It is useful after changing PATH or installing a release build.

## 14. VS Code Support

The VS Code package is located at:

```text
editors/vscode/lizard-vscode
```

It currently provides:

- `.lz` language registration
- syntax highlighting
- comments, brackets, and indentation rules
- snippets for functions, conditions, loops, and `each`
- LIZARD file icon
- Run, Check, and Format commands
- formatting through the real LIZARD CLI

Build the extension after installing Node.js and npm:

```powershell
cd editors/vscode/lizard-vscode
npm install
npm run package
```

Configure a custom executable path in VS Code settings:

```json
{
  "lizard.path": "C:/path/to/lizard.exe",
  "lizard.formatOnSave": true
}
```

## 15. Tests and Validation

Format and compile the Rust project:

```powershell
cargo fmt -- --check
cargo check
```

Run all integration tests when a native linker is available:

```powershell
cargo test
```

The tests cover:

- hello-world execution
- arithmetic and conditions
- functions and repeat loops
- lists, maps, indexing, and `each`
- CLI version/help
- inline execution
- check and format commands
- native build output

## 16. Common Problems

### `lz.exe` does not exist

The release build did not complete. Check the linker first:

```powershell
x86_64-w64-mingw32-gcc --version
```

Then run:

```powershell
.\build.ps1
```

### `link.exe not found`

The MSVC linker is missing. Install Visual Studio Build Tools with the Desktop development with C++ workload, or use the GNU target with MinGW.

### `x86_64-w64-mingw32-gcc not found`

MinGW/WinLibs is missing or not on PATH. Add its `bin` directory to the current PATH, reopen the terminal, and retry the build.

### `lz` is not recognized

The release directory is not on PATH, or the terminal was opened before PATH was changed. Add the directory using the installation instructions and open a new terminal.

### `cargo check` passes but `cargo test` cannot link

Rust source compilation can succeed while executable linking fails. Install a compatible native linker, then rerun `cargo test`.

## 17. Current Scope

The current implementation is a real native interpreter and CLI foundation. The parser/runtime and initial VS Code package are implemented. Full LSP, DAP debugger, package manager, large standard library, and integrations for every editor listed in the larger roadmap are future milestones, not currently claimed as complete.
