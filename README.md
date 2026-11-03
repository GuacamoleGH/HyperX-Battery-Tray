# HyperX Battery Tray

> A lightweight Windows system-tray battery indicator for **HyperX Cloud III S Wireless** — no NGENUITY, no HyperX background app.

[![Release](https://img.shields.io/github/v/release/GuacamoleGH/HyperX-Battery-Tray?display_name=tag&sort=semver&color=ef4444)](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest)
[![License](https://img.shields.io/github/license/GuacamoleGH/HyperX-Battery-Tray?color=22c55e)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078D4)](https://www.microsoft.com/windows)

Keep your headset battery level visible at a glance, directly in the Windows notification area.

## Highlights

| | Feature |
| --- | --- |
| 🔋 | Reads the headset battery through the USB dongle — **without NGENUITY**. |
| 🎨 | Three tray display modes: battery + percentage, battery only, or percentage only. |
| 🟢 | Green, amber, red, and grey states make battery level immediately readable. |
| ⚙️ | Select a refresh period and optionally start automatically with Windows. |
| 🛠️ | Includes `hyperx-probe.exe` to diagnose HID connections. |

## Install

1. Download `HyperXBatteryTray-Setup-*.exe` from [**Releases**](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest).
2. Run the installer. It installs only for the current user and does not require administrator rights.
3. Connect the USB dongle and turn on the headset.

> [!WARNING]
> Executables are currently unsigned. SmartScreen or antivirus software may show a warning or false positive. Download only from the official Releases page and verify `SHA256SUMS.txt` when needed.

## Tray menu

Right-click the tray icon to access:

- **Status** — the latest battery state.
- **Refresh now** — read the headset immediately.
- **Show in tray** — choose battery + percentage, battery only, or percentage only.
- **Refresh every** — choose 30 seconds, 1 minute, 2 minutes, or 5 minutes.
- **Start with Windows** — enable or disable per-user autostart.
- **Exit** — close the application.

The tooltip always shows the current level, for example `HyperX Cloud III S: 71%`.

## Status colours

| Colour | Meaning |
| --- | --- |
| 🟢 Green | Normal battery level |
| 🟠 Amber | 40% or lower |
| 🔴 Red | Low-battery threshold (20% by default) |
| ⚪ Grey | No current battery data |

## If it does not work

Run the diagnostic tool from a terminal:

```powershell
.\hyperx-probe.exe
```

It lists HyperX HID interfaces, identifies the selected interface, and prints the battery percentage and voltage when available. Open an issue with that output if the dongle is not detected.

The application stores settings in `%APPDATA%\HyperXBatteryTray\settings.json` and rotates its log at 256 KB in the same folder.

## How it works

This project uses an **unofficial** HID protocol, reported in a HeadsetControl issue comment. It looks for VID `03F0`, PID `02CC` or `06BE`, `usage_page 0x01C0`, and `usage 0x0001`; then sends `0C 02 03 01 00 06`. The percentage is returned in byte 6.

Voltage is accepted only between 2.5 V and 5 V. Charging state is not currently available. This is not an official HyperX protocol.

## Development

See [BUILD.md](BUILD.md) for requirements, manual builds, installer generation, and release publishing.

Inspired by [razer-battery-report](https://github.com/xzeldon/razer-battery-report). Licensed under [MIT](LICENSE) — © GuacamoleGH.
