# HyperX Battery Tray

Indicador de batería en la bandeja de Windows 10/11 x64 para los auriculares HyperX Cloud III S Wireless. No instala ni necesita NGENUITY ni la aplicación de HyperX.

## Instalación

Descarga `HyperXBatteryTray-Setup-*.exe` de [la última versión](https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest), ejecútalo y conecta el dongle USB. El instalador funciona por usuario y puede iniciar la aplicación con Windows.

Los ejecutables no están firmados. Windows SmartScreen o un antivirus pueden mostrar una advertencia o falso positivo; descarga únicamente desde las Releases oficiales y comprueba `SHA256SUMS.txt`.

## Uso

El icono puede mostrar batería y porcentaje (predeterminado), solo batería o solo número. Verde indica nivel normal, ámbar 40% o menos, rojo el umbral bajo (20% predeterminado) y gris que no hay datos. El tooltip muestra `HyperX Cloud III S: 71%`.

Los estados posibles son: `Batería: 71%`, `Auriculares apagados o sin enlace`, `Dongle USB no detectado`, `Interfaz HID no encontrada (ver registro)`, `Sin respuesta de los auriculares` y `Error leyendo la batería (ver registro)`.

La configuración se guarda en `%APPDATA%\HyperXBatteryTray\settings.json` y el registro, con rotación a 256 KB, en `log.txt`.

## Protocolo

Es un protocolo HID **no oficial**, procedente de un comentario del issue 486 de HeadsetControl. Se filtra el VID `03F0`, PID `02CC/06BE`, `usage_page 0x01C0` y `usage 0x0001`; se envía `0C 02 03 01 00 06` y se recibe el porcentaje en el byte 6. La tensión solo se acepta entre 2,5 y 5 V. El estado de carga aún no se conoce.

Si no funciona, ejecuta `hyperx-probe.exe` desde una consola y adjunta su lista de interfaces y salida al crear un issue. Prueba con los auriculares encendidos y el dongle directamente conectado.

## Estructura

- `src/headset.rs`: protocolo HID y lectura resiliente a reconexiones.
- `src/icons.rs`: iconos de píxeles generados en memoria.
- `src/bin/probe.rs`: diagnóstico de consola.
- `installer/`: instalador Inno Setup.

Inspirado por [razer-battery-report](https://github.com/xzeldon/razer-battery-report). Código bajo licencia [MIT](LICENSE), © GuacamoleGH.
