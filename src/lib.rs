//! Shared components for the application and diagnostic utility.
pub mod headset;
pub mod icons;
pub mod logger;
pub mod settings;
#[cfg(windows)]
pub mod autostart;
#[cfg(windows)]
pub mod updater;
