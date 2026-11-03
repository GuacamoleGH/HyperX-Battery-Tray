param([string]$Version = "0.1.0", [switch]$SkipInstaller)
$ErrorActionPreference = 'Stop'
cargo test --release --locked
cargo build --release --locked
New-Item -ItemType Directory -Force dist | Out-Null
Copy-Item target\release\HyperXBatteryTray.exe dist\
Copy-Item target\release\hyperx-probe.exe dist\
if (-not $SkipInstaller) {
  $iscc = (Get-Command ISCC.exe -ErrorAction SilentlyContinue).Source
  if (-not $iscc) { throw 'ISCC.exe was not found. Install Inno Setup 6.' }
  & $iscc "/DMyAppVersion=$Version" installer\hyperx-battery-tray.iss
}
Get-ChildItem dist -File | Get-FileHash -Algorithm SHA256 | ForEach-Object { "{0} *{1}" -f $_.Hash, $_.Path.Split('\')[-1] } | Set-Content dist\SHA256SUMS.txt
