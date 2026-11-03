# HyperX Battery Tray

A Windows 10/11 x64 system-tray battery indicator for HyperX Cloud III S Wireless headsets. It does not install or require NGENUITY or the HyperX application.

## Installation

Download `HyperXBatteryTray-Setup-*.exe` from the [latest release](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest), run it, and connect the USB dongle. The installer works per user and can start the application with Windows.

The executables are unsigned. Windows SmartScreen or antivirus software may show a warning or false positive; only download from the official Releases and verify `SHA256SUMS.txt`.

## Usage

The icon can show a battery with percentage (default), battery only, or a large number only. Green means normal level, amber means 40% or lower, red means the low threshold (20% by default), and grey means no data. The tooltip reads `HyperX Cloud III S: 71%`.

Possible states are: `Battery: 71%`, `Headset is powered off or disconnected`, `USB dongle not detected`, `HID interface not found (see log)`, `No response from headset`, and `Battery read error (see log)`.

Settings are saved in `%APPDATA%\HyperXBatteryTray\settings.json`; logs rotate at 256 KB and are stored in `log.txt`.

## Protocol

This is an **unofficial** HID protocol from a comment on HeadsetControl issue 486. The application filters VID `03F0`, PID `02CC/06BE`, `usage_page 0x01C0`, and `usage 0x0001`; it sends `0C 02 03 01 00 06` and receives the percentage in byte 6. Voltage is accepted only between 2.5 and 5 V. Charging state is not known yet.

If it does not work, run `hyperx-probe.exe` in a console and include its interface list and output when opening an issue. Use it with the headset powered on and the dongle directly connected.

## Structure

- `src/headset.rs`: HID protocol and USB-reconnect-tolerant reads.
- `src/icons.rs`: in-memory pixel-generated icons.
- `src/bin/probe.rs`: console diagnostics.
- `installer/`: Inno Setup installer.

Inspired by [razer-battery-report](https://github.com/xzeldon/razer-battery-report). Released under the [MIT License](LICENSE), © GuacamoleGH.
