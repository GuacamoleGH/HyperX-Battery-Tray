#![cfg_attr(windows, windows_subsystem = "windows")]
#[cfg(windows)] mod app;
#[cfg(windows)] fn main(){ app::run(); }
#[cfg(not(windows))] fn main(){ eprintln!("HyperXBatteryTray solo se ejecuta en Windows."); }
