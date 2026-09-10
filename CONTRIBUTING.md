# Contributing to LIZARD

Thank you for helping improve LIZARD.

## Project rules

- Keep LIZARD a native implementation. Do not replace runtime behavior with a Python wrapper, transpiler, mock, or documentation-only feature.
- Every language or runtime change must include a focused real-behavior test.
- New built-ins, syntax, animations, and CLI behavior must include executable
  coverage. Do not add a function name without implementing and testing it.
- Run the Windows-target test suite before submitting a change:

  ```powershell
  $env:Path = "C:\mingw64\bin;$env:Path"
  cargo test --target x86_64-pc-windows-gnu -q
  ```

- Keep cross-platform release changes compatible with Windows, Linux, and
  macOS. Release packaging is defined in `.github/workflows/release.yml`.
- Keep public syntax and builtin behavior documented in `README.md` or `docs/LIZARD-GUIDE.md`.
- Document animation frame counts, delays, terminal behavior, and any platform
  limitations when adding animated output.
- Preserve ASCII source files unless Unicode is required by the feature.
- Do not commit `target/`, temporary files, credentials, or unrelated generated output.
- Keep changes focused. Do not rewrite unrelated parser, runtime, or documentation code.
- Use clear commit messages and explain user-visible behavior in pull requests.

## Pull requests

A pull request should include:

1. A short description of the behavior changed.
2. Tests covering the changed behavior.
3. Documentation updates when syntax, CLI commands, builtins, animations, or installation changes.
4. Validation output or a note explaining any unavailable toolchain.

## Language design

New syntax should fit LIZARD's simple block-based style. Prefer a small, composable feature with clear runtime errors over a large untested API surface.

Feature work should preserve the native Rust runtime and should not introduce a
platform-specific implementation when a portable behavior is practical.

## License

By contributing to LIZARD, you agree that your contributions are provided under the MIT License in `LICENSE`.
