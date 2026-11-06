use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMode {
    #[default]
    #[serde(alias = "icon_percent")]
    Icon,
    #[serde(alias = "vertical_percent")]
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Settings {
    pub display_mode: DisplayMode,
    pub interval_s: u64,
    pub low_threshold: u8,
    pub notify_low: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            display_mode: DisplayMode::Icon,
            interval_s: 60,
            low_threshold: 20,
            notify_low: true,
        }
    }
}

impl Settings {
    pub fn normalized(mut self) -> Self {
        if ![30, 60, 120, 300].contains(&self.interval_s) {
            self.interval_s = 60;
        }
        if self.low_threshold > 100 {
            self.low_threshold = 20;
        }
        self
    }

    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str::<Self>(&s).ok())
            .unwrap_or_default()
            .normalized()
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_icon() {
        assert_eq!(DisplayMode::default(), DisplayMode::Icon);
    }

    #[test]
    fn deserializes_legacy_modes() {
        let json_horiz = r#"{"display_mode":"icon_percent","interval_s":60,"low_threshold":20,"notify_low":true}"#;
        let s_horiz: Settings = serde_json::from_str(json_horiz).unwrap();
        assert_eq!(s_horiz.display_mode, DisplayMode::Icon);

        let json_vert = r#"{"display_mode":"vertical_percent","interval_s":60,"low_threshold":20,"notify_low":true}"#;
        let s_vert: Settings = serde_json::from_str(json_vert).unwrap();
        assert_eq!(s_vert.display_mode, DisplayMode::Number);
    }
}
