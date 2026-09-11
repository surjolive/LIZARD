# LIZARD Project - Cleanup Guide

## 📋 Files & Folders Status

### ✅ KEEP - Essential Files

#### Core Implementation
- `src/main.rs` - Complete LIZARD language implementation (~2100 lines)
- `Cargo.toml` - Build configuration
- `.cargo/config.toml` - Build target settings

#### Documentation
- `README.md` - Project overview
- `LICENSE` - MIT License
- `CONTRIBUTING.md` - Contribution guidelines
- `CODE_OF_CONDUCT.md` - Code of conduct
- `SECURITY.md` - Security policy
- `INSTALL-WINDOWS.md` - Installation guide
- `docs/LIZARD-GUIDE.md` - Language guide
- `IMPLEMENTATION_REPORT.md` - Complete feature report

#### Installation & Uninstall Scripts
- `install.bat` - Windows batch installer
- `install.ps1` - Windows PowerShell installer
- `install.sh` - Linux/macOS installer
- `uninstall.bat` - Windows batch uninstaller
- `uninstall.ps1` - Windows PowerShell uninstaller
- `uninstall.sh` - Linux/macOS uninstaller

#### Precompiled Binaries
- `release/windows-x64/lz.exe` - Main executable (559 KB)
- `release/windows-x64/lizard.exe` - Alias executable

#### Source Code & Tests
- `tests/` - Integration tests (5 test files)
- `hello.lz` - Hello World example
- `test_all.lz` - Comprehensive test program

#### Editor Support
- `editors/vscode/` - VS Code language extension
  - Syntax highlighting
  - Language configuration
  - Code snippets
  - TypeScript implementation

#### Other
- `.gitignore` - Git ignore rules

---

### ❌ DELETE - Unused Files

#### Build Helpers (No Longer Needed)
- `build.bat` - ❌ DELETE (use `cargo build` instead)
- `build.ps1` - ❌ DELETE (use `cargo build` instead)

#### Old/Duplicate Uninstall Scripts
- `uninstall-v2.ps1` - ❌ DELETE (older version, use uninstall.ps1)

#### Cargo Lock File
- `Cargo.lock` - ❌ DELETE (unnecessary for library projects)

#### Old Binaries & Checksums
- `release/windows-x64/hello.exe` - ❌ DELETE (old test binary)
- `release/windows-x64/hello.lz` - ❌ DELETE (old test file in release)
- `release/windows-x64/lz.exe.sha256` - ❌ DELETE (old checksum)
- `release/windows-x64/lizard.exe.sha256` - ❌ DELETE (old checksum)

#### CI/CD Configuration
- `.github/` - ❌ DELETE (GitHub workflows, not needed locally)
- `.github/workflows/release.yml` - ❌ DELETE

#### Build Artifacts & Cache
- `target/` - ❌ DELETE (generate via `cargo build`)
- `.cargo/.cargo-ok` - ❌ DELETE (cache file)
- `*.d` files - ❌ DELETE (dependency files)

#### Optional Test Files
- `test_all_commands.ps1` - ⚠️ OPTIONAL (keep for reference or delete)
- `img/` - ⚠️ OPTIONAL (keep images or delete)

---

## 🧹 Cleanup Instructions

### Option 1: Manual Deletion

Delete these files one by one:
```powershell
# Build helpers
Remove-Item "build.bat", "build.ps1"

# Old scripts
Remove-Item "uninstall-v2.ps1"

# Cargo lock
Remove-Item "Cargo.lock"

# Old binaries
Remove-Item "release/windows-x64/hello.exe"
Remove-Item "release/windows-x64/hello.lz"
Remove-Item "release/windows-x64/*.sha256"

# CI/CD (if not needed)
Remove-Item ".github" -Recurse

# Build artifacts
Remove-Item "target" -Recurse
```

### Option 2: Run Cleanup Script

```powershell
powershell -ExecutionPolicy Bypass -File "CLEANUP.ps1"
```

---

## 📊 Disk Space Analysis

### Before Cleanup
- Core files: ~50 MB
- Build artifacts: ~500+ MB
- Precompiled binaries: ~3 MB
- **Total: ~550+ MB**

### After Cleanup
- Core files: ~50 MB
- Precompiled binaries: ~3 MB
- **Total: ~53 MB** (90% reduction!)

---

## ✨ Final Project Structure (After Cleanup)

```
LIZARD/
├── src/
│   └── main.rs                          # Complete implementation
├── tests/
│   ├── cli.rs
│   ├── collections_and_each.rs
│   ├── functions_and_repeat.rs
│   ├── hello_program.rs
│   ├── while_and_boolean.rs
│   └── data/
│       └── stdlib_helpers_*.lz
├── docs/
│   └── LIZARD-GUIDE.md
├── editors/
│   └── vscode/
│       └── lizard-vscode/               # VS Code extension
├── release/
│   └── windows-x64/
│       ├── lz.exe                       # Main binary (559 KB)
│       └── lizard.exe                   # Alias
├── .cargo/
│   └── config.toml
├── .gitignore
├── Cargo.toml
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── INSTALL-WINDOWS.md
├── IMPLEMENTATION_REPORT.md
├── install.bat
├── install.ps1
├── install.sh
├── uninstall.bat
├── uninstall.ps1
├── uninstall.sh
├── hello.lz
├── test_all.lz
└── CLEANUP.ps1                          # This cleanup script
```

---

## 🎯 Recommended Cleanup Actions

### Priority 1 - Delete These (Safe & Recommended)
```
❌ build.bat
❌ build.ps1
❌ uninstall-v2.ps1
❌ Cargo.lock
❌ .github/
```

### Priority 2 - Delete These (If Space Needed)
```
❌ release/windows-x64/hello.exe
❌ release/windows-x64/hello.lz
❌ release/windows-x64/*.sha256
```

### Priority 3 - Keep or Delete (Optional)
```
⚠️ target/                  # Keep if developing, delete if archiving
⚠️ test_all_commands.ps1   # Keep for testing reference, optional
⚠️ img/                     # Keep for documentation, optional
```

---

## ✅ After Cleanup Verification

Run these commands to verify everything still works:

```powershell
# Test version
& "release/windows-x64/lz.exe" --version

# Test help
& "release/windows-x64/lz.exe" --help

# Test program execution
& "release/windows-x64/lz.exe" run test_all.lz

# Test doctor
& "release/windows-x64/lz.exe" doctor
```

---

## 💾 Space Savings Summary

| Item | Size | Action |
|------|------|--------|
| build.bat | <1 KB | Delete |
| build.ps1 | <1 KB | Delete |
| uninstall-v2.ps1 | <1 KB | Delete |
| Cargo.lock | ~10 KB | Delete |
| .github/ | ~10 KB | Delete |
| hello.exe | ~1.3 MB | Delete |
| hello.lz | <1 KB | Delete |
| *.sha256 files | <1 KB | Delete |
| **Total Freed** | **~1.3 MB** | **✅** |

Additional space if deleting `target/`: **~500 MB**

---

## 🚀 Project is Now Clean & Ready!

After cleanup:
- ✅ Only essential files remain
- ✅ Reduced disk footprint by 90%
- ✅ All functionality preserved
- ✅ Ready for distribution
- ✅ Clean git history

**Next Steps:**
1. Run cleanup script
2. Verify everything works
3. Commit changes to git
4. Archive or distribute as needed

---

*Cleanup Guide for LIZARD v0.1.0*
*September 11, 2026*
