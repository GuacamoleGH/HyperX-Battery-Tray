# Compilar y publicar

## Requisitos

- Windows 10/11 x64.
- Rust estable con destino MSVC y Visual Studio Build Tools (C++).
- Inno Setup 6 para crear el instalador.

## Script

En PowerShell:

```powershell
.\build.ps1 -Version 0.1.0
```

Ejecuta las pruebas en release, compila con `--locked`, coloca los ejecutables en `dist`, crea el instalador y escribe `dist\SHA256SUMS.txt`. Usa `-SkipInstaller` si no tienes Inno Setup.

Manual: `cargo test --release --locked` y `cargo build --release --locked`.

## Publicación

Tras validar localmente, actualiza la versión de `Cargo.toml`, crea y sube una etiqueta `vX.Y.Z`. El workflow de GitHub Actions comprueba que etiqueta y versión coinciden, instala Inno Setup y publica los cuatro archivos en [Releases](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest).

Problemas habituales: instala Build Tools si `link.exe` no aparece; instala Inno Setup si falta `ISCC.exe`; y usa `hyperx-probe.exe` para comprobar si el dongle expone la colección HID esperada.
