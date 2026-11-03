# Build and release

## Requirements

- Windows 10/11 x64.
- Stable Rust with the MSVC target and Visual Studio Build Tools (C++).
- Inno Setup 6 to build the installer.

## Script

In PowerShell:

```powershell
.\build.ps1 -Version 0.1.0
```

It runs release tests, builds with `--locked`, places the executables in `dist`, creates the installer, and writes `dist\SHA256SUMS.txt`. Use `-SkipInstaller` when Inno Setup is unavailable.

For a manual build, run `cargo test --release --locked` and `cargo build --release --locked`.

## Publishing

After local validation, update `Cargo.toml`, create and push a `vX.Y.Z` tag. GitHub Actions verifies that the tag matches the Cargo version, installs Inno Setup, and publishes the four artifacts on [Releases](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest).

Common problems: install Build Tools if `link.exe` is missing; install Inno Setup if `ISCC.exe` is missing; use `hyperx-probe.exe` to verify that the dongle exposes the expected HID collection.
