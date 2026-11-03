//! Unofficial HID protocol for the HyperX Cloud III S Wireless dongle.
use hidapi::{HidApi, HidDevice, DeviceInfo};
use std::time::{Duration, Instant};

pub const VID: u16 = 0x03f0;
pub const PIDS: [u16; 2] = [0x02cc, 0x06be];
pub const USAGE_PAGE: u16 = 0x01c0;
pub const USAGE: u16 = 0x0001;
pub const REPORT_ID: u8 = 0x0c;
pub const OP_BATTERY: u8 = 0x06;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryReading { pub percent: u8, pub millivolts: Option<u16> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    DongleNotFound,
    InterfaceNotFound,
    NoResponse,
    Disconnected,
    Hid(String),
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::DongleNotFound => write!(f, "USB dongle not detected"), Self::InterfaceNotFound => write!(f, "HID interface not found"), Self::NoResponse => write!(f, "No response from headset"), Self::Disconnected => write!(f, "Headset is powered off or disconnected"), Self::Hid(e) => write!(f, "Battery read error: {e}") }
    }
}
impl std::error::Error for ReadError {}

pub fn is_target(info: &DeviceInfo) -> bool {
    PIDS.contains(&info.product_id()) && info.vendor_id() == VID && info.usage_page() == USAGE_PAGE && info.usage() == USAGE
}
pub fn parse_battery_frame(data: &[u8]) -> Option<Result<BatteryReading, ReadError>> {
    if data.len() < 7 || data[0] != REPORT_ID || data[5] != OP_BATTERY { return None; }
    if data[6] > 100 { return Some(Err(ReadError::Disconnected)); }
    let millivolts = if data.len() > 8 { let mv = u16::from_be_bytes([data[7], data[8]]); (2500..=5000).contains(&mv).then_some(mv) } else { None };
    Some(Ok(BatteryReading { percent: data[6], millivolts }))
}

fn send_command(device: &HidDevice) -> Result<(), ReadError> {
    for size in [64usize, 52, 65] {
        let mut buf = vec![0u8; size];
        buf[..6].copy_from_slice(&[REPORT_ID, 0x02, 0x03, 0x01, 0x00, OP_BATTERY]);
        if device.write(&buf).map_err(|e| ReadError::Hid(e.to_string()))? > 0 { return Ok(()); }
    }
    Err(ReadError::NoResponse)
}
fn drain(device: &HidDevice) { let mut b = [0; 64]; while device.read_timeout(&mut b, 0).unwrap_or(0) > 0 {} }

/// Creates HidApi and opens/closes the device per read to tolerate USB reconnects.
pub fn read_battery() -> Result<BatteryReading, ReadError> {
    let api = HidApi::new().map_err(|e| ReadError::Hid(e.to_string()))?;
    let candidates: Vec<_> = api.device_list().filter(|d| d.vendor_id() == VID && PIDS.contains(&d.product_id())).collect();
    if candidates.is_empty() { return Err(ReadError::DongleNotFound); }
    let info = candidates.into_iter().find(|d| is_target(d)).ok_or(ReadError::InterfaceNotFound)?;
    let device = info.open_device(&api).map_err(|e| ReadError::Hid(e.to_string()))?;
    drain(&device); send_command(&device)?;
    let deadline = Instant::now() + Duration::from_secs(2); let mut buf = [0; 64];
    while Instant::now() < deadline {
        let n = device.read_timeout(&mut buf, 200).map_err(|e| ReadError::Hid(e.to_string()))?;
        if let Some(value) = parse_battery_frame(&buf[..n]) { return value; }
    }
    Err(ReadError::NoResponse)
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn captura_real_se_parsea() { let d=[0x0c,2,3,1,0,6,71,0x0f,0x40]; assert_eq!(parse_battery_frame(&d),Some(Ok(BatteryReading{percent:71,millivolts:Some(3904)}))); }
    #[test] fn frames_ajenos_se_rechazan() { assert_eq!(parse_battery_frame(&[0,2,3,1,0,6,71]),None); assert_eq!(parse_battery_frame(&[12,2,3,1,0,7,71]),None); }
    #[test] fn tension_inverosimil_se_descarta() { let d=[12,2,3,1,0,6,50,0,1]; assert_eq!(parse_battery_frame(&d).unwrap().unwrap().millivolts,None); }
}
