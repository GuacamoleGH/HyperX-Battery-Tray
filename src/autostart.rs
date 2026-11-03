use std::path::Path;
use winreg::{enums::HKEY_CURRENT_USER,RegKey};
const KEY:&str="Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const NAME:&str="HyperXBatteryTray";
pub fn enabled()->bool{RegKey::predef(HKEY_CURRENT_USER).open_subkey(KEY).ok().and_then(|k|k.get_value::<String,_>(NAME).ok()).is_some()}
pub fn set(enabled:bool,exe:&Path)->std::io::Result<()> {let key=RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(KEY,winreg::enums::KEY_WRITE)?;if enabled{key.set_value(NAME,&format!("\"{}\"",exe.display()))?;}else{let _=key.delete_value(NAME);}Ok(())}
