//! Automatic update checker using GitHub Releases.

use std::{os::windows::process::CommandExt, process::Command};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateInfo {
    pub tag: String,
    pub url: String,
}

/// Parses a semver string like "v0.1.8" or "0.1.9" into (major, minor, patch).
pub fn parse_version(s: &str) -> Option<(u32, u32, u32)> {
    let clean = s.trim().trim_start_matches('v');
    let mut parts = clean.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

/// Returns true if `remote` is strictly newer than `current`.
pub fn is_newer(remote: &str, current: &str) -> bool {
    match (parse_version(remote), parse_version(current)) {
        (Some((r_maj, r_min, r_pat)), Some((c_maj, c_min, c_pat))) => {
            (r_maj, r_min, r_pat) > (c_maj, c_min, c_pat)
        }
        _ => false,
    }
}

/// Queries GitHub Releases for the latest version tag.
pub fn check_for_updates() -> Result<Option<UpdateInfo>, String> {
    let output = Command::new("curl.exe")
        .args([
            "-s",
            "--max-time",
            "10",
            "-H",
            "User-Agent: HyperXBatteryTray",
            "https://api.github.com/repos/GuacamoleGH/HyperX-Battery-Tray/releases/latest",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to run curl: {e}"))?;

    if !output.status.success() {
        return Err("GitHub release request failed".to_string());
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Invalid JSON response: {e}"))?;

    let tag = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "No tag_name in response".to_string())?;

    let url = json
        .get("html_url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest")
        .to_string();

    let current = env!("CARGO_PKG_VERSION");
    if is_newer(tag, current) {
        Ok(Some(UpdateInfo {
            tag: tag.to_string(),
            url,
        }))
    } else {
        Ok(None)
    }
}

/// Opens a URL in the user's default browser silently.
pub fn open_url(url: &str) {
    let _ = Command::new("cmd.exe")
        .args(["/C", "start", "", url])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

/// Prompts the user with a Yes/No dialog asking if they want to download the update.
pub fn show_update_prompt(tag: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_ICONINFORMATION, MB_YESNO,
    };

    let title = to_wide("HyperX Battery Tray - Update Available");
    let message = to_wide(&format!(
        "A new version of HyperX Battery Tray ({tag}) is available!\n\nWould you like to open the download page now?"
    ));
    // MB_YESNO (0x4) | MB_ICONINFORMATION (0x40) | MB_SETFOREGROUND (0x10000) | MB_TOPMOST (0x40000)
    let flags = MB_YESNO | MB_ICONINFORMATION | 0x0001_0000 | 0x0004_0000;

    unsafe {
        let result = MessageBoxW(core::ptr::null_mut(), message.as_ptr(), title.as_ptr(), flags);
        result == IDYES
    }
}

/// Shows an informational message that the app is up to date.
pub fn show_up_to_date_message(current: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    let title = to_wide("HyperX Battery Tray");
    let message = to_wide(&format!(
        "You are running the latest version of HyperX Battery Tray (v{current})."
    ));
    let flags = MB_OK | MB_ICONINFORMATION | 0x0001_0000;

    unsafe {
        let _ = MessageBoxW(core::ptr::null_mut(), message.as_ptr(), title.as_ptr(), flags);
    }
}

/// Shows an error message if update check failed.
pub fn show_error_message(err: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONWARNING, MB_OK};

    let title = to_wide("HyperX Battery Tray - Update Check");
    let message = to_wide(&format!(
        "Could not check for updates:\n{err}\n\nPlease check your internet connection."
    ));
    let flags = MB_OK | MB_ICONWARNING | 0x0001_0000;

    unsafe {
        let _ = MessageBoxW(core::ptr::null_mut(), message.as_ptr(), title.as_ptr(), flags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        assert_eq!(parse_version("v0.1.8"), Some((0, 1, 8)));
        assert_eq!(parse_version("0.1.9"), Some((0, 1, 9)));
        assert_eq!(parse_version("v1.2.3"), Some((1, 2, 3)));
    }

    #[test]
    fn test_is_newer() {
        assert!(is_newer("v0.1.9", "0.1.8"));
        assert!(is_newer("v0.2.0", "0.1.8"));
        assert!(is_newer("v1.0.0", "0.1.8"));
        assert!(!is_newer("v0.1.8", "0.1.8"));
        assert!(!is_newer("v0.1.7", "0.1.8"));
        assert!(!is_newer("v0.0.9", "0.1.8"));
    }
}
