use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode { #[default] IconPercent, Icon, Number }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Settings { pub display_mode: DisplayMode, pub interval_s: u64, pub low_threshold: u8, pub notify_low: bool }
impl Default for Settings { fn default() -> Self { Self { display_mode: DisplayMode::IconPercent, interval_s: 60, low_threshold: 20, notify_low: true } } }
impl Settings { pub fn normalized(mut self) -> Self { if ![30,60,120,300].contains(&self.interval_s) { self.interval_s=60; } if self.low_threshold>100 { self.low_threshold=20; } self }
pub fn load(path:&Path)->Self { fs::read_to_string(path).ok().and_then(|s|serde_json::from_str::<Self>(&s).ok()).unwrap_or_default().normalized() }
pub fn save(&self,path:&Path)->std::io::Result<()> { if let Some(p)=path.parent(){fs::create_dir_all(p)?}; fs::write(path,serde_json::to_string_pretty(&self.clone().normalized()).unwrap()) } }
#[cfg(test)] mod tests { use super::*; #[test] fn basura_defecto(){let p=std::env::temp_dir().join("hyperx-settings-basura.json");fs::write(&p,"{").unwrap();assert_eq!(Settings::load(&p),Settings::default());let _=fs::remove_file(p);} #[test] fn json_compatible(){let s:Settings=serde_json::from_str(r#"{"display_mode":"number","interval_s":120,"low_threshold":15,"notify_low":false}"#).unwrap();assert_eq!(s.display_mode,DisplayMode::Number);} }
