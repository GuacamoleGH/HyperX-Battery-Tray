#define MyAppName "HyperX Battery Tray"
#define MyAppVersion GetStringParam('MyAppVersion', '0.1.0')
#define MyAppExeName "HyperXBatteryTray.exe"
[Setup]
AppId={{7FA5A9E8-9D35-4E20-B79B-EC26E6D96514}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
DefaultDirName={autopf}\HyperXBatteryTray
DefaultGroupName={#MyAppName}
PrivilegesRequired=lowest
OutputDir=..\dist
OutputBaseFilename=HyperXBatteryTray-Setup-{#MyAppVersion}
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"
[Tasks]
Name: "autostart"; Description: "Start with Windows"; GroupDescription: "Options:"; Flags: checkedonce
Name: "desktopicon"; Description: "Create a desktop shortcut"; Flags: unchecked
[Files]
Source: "..\target\release\HyperXBatteryTray.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\hyperx-probe.exe"; DestDir: "{app}"; Flags: ignoreversion
[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "HyperXBatteryTray"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: autostart; Flags: uninsdeletevalue
[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Iniciar HyperX Battery Tray"; Flags: nowait postinstall skipifsilent
[InstallDelete]
Type: files; Name: "{app}\HyperXBatteryTray.exe"
[UninstallRun]
Filename: "taskkill"; Parameters: "/f /im HyperXBatteryTray.exe"; Flags: runhidden
