use hyperx_battery::{autostart, headset, icons, settings::{DisplayMode, Settings}};
use std::{path::PathBuf, sync::{Arc, atomic::{AtomicU64, Ordering}}, thread, time::Duration};
use tao::{event::{Event, StartCause}, event_loop::{ControlFlow, EventLoopBuilder}};
use tray_icon::{menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu}, TrayIcon, TrayIconBuilder};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSMICON};

#[derive(Clone, Debug)]
enum UserEvent { Reading(Result<headset::BatteryReading, headset::ReadError>), Refresh, SetMode(DisplayMode), SetInterval(u64), ToggleAutostart, Quit }

fn tray_size() -> u32 { unsafe { GetSystemMetrics(SM_CXSMICON).max(32) as u32 } }
fn settings_path() -> PathBuf { std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join("HyperXBatteryTray").join("settings.json") }

pub fn run() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let settings_file = settings_path();
    let mut settings = Settings::load(&settings_file);
    let menu = Menu::new();
    let status = MenuItem::with_id("status", "Status: no data", false, None);
    let refresh = MenuItem::with_id("refresh", "Refresh now", true, None);
    let show = Submenu::new("Show in tray", true);
    let show_percent = CheckMenuItem::with_id("mode_percent", "Battery and percentage", true, settings.display_mode == DisplayMode::IconPercent, None);
    let show_icon = CheckMenuItem::with_id("mode_icon", "Battery only", true, settings.display_mode == DisplayMode::Icon, None);
    let show_number = CheckMenuItem::with_id("mode_number", "Percentage only", true, settings.display_mode == DisplayMode::Number, None);
    show.append_items(&[&show_percent, &show_icon, &show_number]).expect("display menu");
    let interval = Submenu::new("Refresh every", true);
    let every_30 = CheckMenuItem::with_id("interval_30", "30 seconds", true, settings.interval_s == 30, None);
    let every_60 = CheckMenuItem::with_id("interval_60", "1 minute", true, settings.interval_s == 60, None);
    let every_120 = CheckMenuItem::with_id("interval_120", "2 minutes", true, settings.interval_s == 120, None);
    let every_300 = CheckMenuItem::with_id("interval_300", "5 minutes", true, settings.interval_s == 300, None);
    interval.append_items(&[&every_30, &every_60, &every_120, &every_300]).expect("interval menu");
    let autostart_item = CheckMenuItem::with_id("autostart", "Start with Windows", true, autostart::enabled(), None);
    let quit = MenuItem::with_id("quit", "Exit", true, None);
    menu.append_items(&[&status, &refresh, &show, &interval, &autostart_item, &PredefinedMenuItem::separator(), &quit]).expect("menu");
    let menu_proxy = proxy.clone();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| { let action = match event.id.as_ref() { "refresh" => UserEvent::Refresh, "mode_percent" => UserEvent::SetMode(DisplayMode::IconPercent), "mode_icon" => UserEvent::SetMode(DisplayMode::Icon), "mode_number" => UserEvent::SetMode(DisplayMode::Number), "interval_30" => UserEvent::SetInterval(30), "interval_60" => UserEvent::SetInterval(60), "interval_120" => UserEvent::SetInterval(120), "interval_300" => UserEvent::SetInterval(300), "autostart" => UserEvent::ToggleAutostart, "quit" => UserEvent::Quit, _ => return }; let _ = menu_proxy.send_event(action); }));
    let initial_proxy = proxy.clone();
    thread::spawn(move || { let _ = initial_proxy.send_event(UserEvent::Reading(headset::read_battery())); });
    let interval_seconds = Arc::new(AtomicU64::new(settings.interval_s));
    let polling_interval = interval_seconds.clone();
    let polling_proxy = proxy.clone();
    thread::spawn(move || loop { thread::sleep(Duration::from_secs(polling_interval.load(Ordering::Relaxed))); let _ = polling_proxy.send_event(UserEvent::Refresh); });
    let mut menu = Some(menu);
    let mut tray: Option<TrayIcon> = None;
    let mut last: Option<u8> = None;
    let mut failures = 0u8;
    let icon_size = tray_size();
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => { let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold); let icon = tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"); tray = Some(TrayIconBuilder::new().with_menu(Box::new(menu.take().expect("menu"))).with_tooltip("HyperX Cloud III S: no data").with_icon(icon).build().expect("tray icon")); }
            Event::UserEvent(UserEvent::Refresh) => { let reader = proxy.clone(); thread::spawn(move || { let _ = reader.send_event(UserEvent::Reading(headset::read_battery())); }); }
            Event::UserEvent(UserEvent::SetMode(mode)) => { settings.display_mode = mode; show_percent.set_checked(mode == DisplayMode::IconPercent); show_icon.set_checked(mode == DisplayMode::Icon); show_number.set_checked(mode == DisplayMode::Number); let _ = settings.save(&settings_file); if let Some(current) = tray.as_ref() { let pixels = icons::make_icon(icon_size, mode, last, settings.low_threshold); let _ = current.set_icon(Some(tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"))); } }
            Event::UserEvent(UserEvent::SetInterval(seconds)) => { settings.interval_s = seconds; interval_seconds.store(seconds, Ordering::Relaxed); every_30.set_checked(seconds == 30); every_60.set_checked(seconds == 60); every_120.set_checked(seconds == 120); every_300.set_checked(seconds == 300); let _ = settings.save(&settings_file); }
            Event::UserEvent(UserEvent::ToggleAutostart) => { let enabled = !autostart::enabled(); if let Ok(exe) = std::env::current_exe() { let _ = autostart::set(enabled, &exe); } autostart_item.set_checked(autostart::enabled()); }
            Event::UserEvent(UserEvent::Quit) => { tray.take(); *flow = ControlFlow::Exit; }
            Event::UserEvent(UserEvent::Reading(result)) => { let state = match result { Ok(value) => { last = Some(value.percent); failures = 0; format!("Battery: {}%", value.percent) }, Err(error) => { failures += 1; if failures > 3 { last = None; } error.to_string() } }; status.set_text(&state); if let Some(current) = tray.as_ref() { let tooltip = last.map(|p| format!("HyperX Cloud III S: {p}%")).unwrap_or_else(|| "HyperX Cloud III S: no data".to_string()); let _ = current.set_tooltip(Some(tooltip)); let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold); let _ = current.set_icon(Some(tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"))); } }
            _ => {}
        }
    });
}
