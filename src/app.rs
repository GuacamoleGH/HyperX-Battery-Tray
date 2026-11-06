use hyperx_battery::{
    autostart, headset, icons,
    settings::{DisplayMode, Settings},
    updater::{self, UpdateInfo},
};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    TrayIcon, TrayIconBuilder,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSMICON};

#[derive(Clone, Debug)]
enum UserEvent {
    Reading(Result<headset::BatteryReading, headset::ReadError>),
    Refresh,
    SetMode(DisplayMode),
    SetInterval(u64),
    SetThreshold(u8),
    ToggleAlerts,
    ToggleAutostart,
    OpenLog,
    CheckUpdates(bool),
    UpdateResult {
        result: Result<Option<UpdateInfo>, String>,
        manual: bool,
    },
    OpenUpdateUrl,
    Quit,
}

/// Draw at the exact notification-area size so Windows does not blur a downscaled icon.
fn tray_size() -> u32 {
    unsafe { GetSystemMetrics(SM_CXSMICON).max(16) as u32 }
}

fn settings_path() -> PathBuf {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("HyperXBatteryTray")
        .join("settings.json")
}

fn format_tooltip(last_battery: Option<u8>, update: &Option<UpdateInfo>) -> String {
    let base = last_battery
        .map(|p| format!("HyperX Cloud III S: {p}%"))
        .unwrap_or_else(|| "HyperX Cloud III S: no data".to_string());
    if let Some(up) = update {
        format!("{base} - Update {} available!", up.tag)
    } else {
        base
    }
}

pub fn run() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let settings_file = settings_path();
    let mut settings = Settings::load(&settings_file);
    let menu = Menu::new();

    let status = MenuItem::with_id("status", "Status: no data", false, None);
    let refresh = MenuItem::with_id("refresh", "Refresh now", true, None);
    let alerts = CheckMenuItem::with_id("alerts", "Battery alerts", true, settings.notify_low, None);

    let low_battery = Submenu::new("Low battery", true);
    let low_10 = CheckMenuItem::with_id("low_10", "10%", true, settings.low_threshold == 10, None);
    let low_20 = CheckMenuItem::with_id("low_20", "20%", true, settings.low_threshold == 20, None);
    let low_30 = CheckMenuItem::with_id("low_30", "30%", true, settings.low_threshold == 30, None);
    low_battery.append_items(&[&low_10, &low_20, &low_30]).expect("low battery menu");

    let show = Submenu::new("Show in tray", true);
    let show_icon = CheckMenuItem::with_id("mode_icon", "Battery only", true, settings.display_mode == DisplayMode::Icon, None);
    let show_number = CheckMenuItem::with_id("mode_number", "Percentage only", true, settings.display_mode == DisplayMode::Number, None);
    show.append_items(&[&show_icon, &show_number]).expect("display menu");

    let interval = Submenu::new("Refresh every", true);
    let every_30 = CheckMenuItem::with_id("interval_30", "30 seconds", true, settings.interval_s == 30, None);
    let every_60 = CheckMenuItem::with_id("interval_60", "1 minute", true, settings.interval_s == 60, None);
    let every_120 = CheckMenuItem::with_id("interval_120", "2 minutes", true, settings.interval_s == 120, None);
    let every_300 = CheckMenuItem::with_id("interval_300", "5 minutes", true, settings.interval_s == 300, None);
    interval.append_items(&[&every_30, &every_60, &every_120, &every_300]).expect("interval menu");

    let autostart_item = CheckMenuItem::with_id("autostart", "Start with Windows", true, autostart::enabled(), None);

    let updates = Submenu::new("Updates", true);
    let update_status = MenuItem::with_id("update_status", "Checking for updates...", false, None);
    let check_updates_item = MenuItem::with_id("check_updates", "Check for updates now", true, None);
    updates.append_items(&[&update_status, &check_updates_item]).expect("updates menu");

    let open_log = MenuItem::with_id("open_log", "Open log", true, None);
    let about = Submenu::new("About", true);
    let about_text = MenuItem::with_id("about_text", concat!("HyperX Battery Tray v", env!("CARGO_PKG_VERSION")), false, None);
    about.append(&about_text).expect("about menu");
    let quit = MenuItem::with_id("quit", "Exit", true, None);

    menu.append_items(&[
        &status,
        &refresh,
        &PredefinedMenuItem::separator(),
        &alerts,
        &low_battery,
        &show,
        &interval,
        &autostart_item,
        &updates,
        &PredefinedMenuItem::separator(),
        &open_log,
        &about,
        &quit,
    ]).expect("menu");

    let menu_proxy = proxy.clone();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        let action = match event.id.as_ref() {
            "refresh" => UserEvent::Refresh,
            "alerts" => UserEvent::ToggleAlerts,
            "low_10" => UserEvent::SetThreshold(10),
            "low_20" => UserEvent::SetThreshold(20),
            "low_30" => UserEvent::SetThreshold(30),
            "mode_icon" => UserEvent::SetMode(DisplayMode::Icon),
            "mode_number" => UserEvent::SetMode(DisplayMode::Number),
            "interval_30" => UserEvent::SetInterval(30),
            "interval_60" => UserEvent::SetInterval(60),
            "interval_120" => UserEvent::SetInterval(120),
            "interval_300" => UserEvent::SetInterval(300),
            "autostart" => UserEvent::ToggleAutostart,
            "check_updates" => UserEvent::CheckUpdates(true),
            "update_status" => UserEvent::OpenUpdateUrl,
            "open_log" => UserEvent::OpenLog,
            "quit" => UserEvent::Quit,
            _ => return,
        };
        let _ = menu_proxy.send_event(action);
    }));

    // Initial battery reading
    let initial_proxy = proxy.clone();
    thread::spawn(move || {
        let _ = initial_proxy.send_event(UserEvent::Reading(headset::read_battery()));
    });

    // Polling battery reading
    let interval_seconds = Arc::new(AtomicU64::new(settings.interval_s));
    let polling_interval = interval_seconds.clone();
    let polling_proxy = proxy.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(polling_interval.load(Ordering::Relaxed)));
        let _ = polling_proxy.send_event(UserEvent::Refresh);
    });

    // Background update check on startup (wait 3 seconds)
    let initial_update_proxy = proxy.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let res = updater::check_for_updates();
        let _ = initial_update_proxy.send_event(UserEvent::UpdateResult {
            result: res,
            manual: false,
        });
    });

    // Periodic update check every 6 hours
    let periodic_update_proxy = proxy.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(6 * 3600));
        let res = updater::check_for_updates();
        let _ = periodic_update_proxy.send_event(UserEvent::UpdateResult {
            result: res,
            manual: false,
        });
    });

    let mut menu = Some(menu);
    let mut tray: Option<TrayIcon> = None;
    let mut last: Option<u8> = None;
    let mut failures = 0u8;
    let mut available_update: Option<UpdateInfo> = None;
    let icon_size = tray_size();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold);
                let icon = tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon");
                tray = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(menu.take().expect("menu")))
                        .with_tooltip("HyperX Cloud III S: no data")
                        .with_icon(icon)
                        .build()
                        .expect("tray icon"),
                );
            }
            Event::UserEvent(UserEvent::Refresh) => {
                let reader = proxy.clone();
                thread::spawn(move || {
                    let _ = reader.send_event(UserEvent::Reading(headset::read_battery()));
                });
            }
            Event::UserEvent(UserEvent::SetMode(mode)) => {
                settings.display_mode = mode;
                show_icon.set_checked(mode == DisplayMode::Icon);
                show_number.set_checked(mode == DisplayMode::Number);
                let _ = settings.save(&settings_file);
                if let Some(current) = tray.as_ref() {
                    let pixels = icons::make_icon(icon_size, mode, last, settings.low_threshold);
                    let _ = current.set_icon(Some(
                        tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"),
                    ));
                }
            }
            Event::UserEvent(UserEvent::SetInterval(seconds)) => {
                settings.interval_s = seconds;
                interval_seconds.store(seconds, Ordering::Relaxed);
                every_30.set_checked(seconds == 30);
                every_60.set_checked(seconds == 60);
                every_120.set_checked(seconds == 120);
                every_300.set_checked(seconds == 300);
                let _ = settings.save(&settings_file);
            }
            Event::UserEvent(UserEvent::SetThreshold(threshold)) => {
                settings.low_threshold = threshold;
                low_10.set_checked(threshold == 10);
                low_20.set_checked(threshold == 20);
                low_30.set_checked(threshold == 30);
                let _ = settings.save(&settings_file);
            }
            Event::UserEvent(UserEvent::ToggleAlerts) => {
                settings.notify_low = !settings.notify_low;
                alerts.set_checked(settings.notify_low);
                let _ = settings.save(&settings_file);
            }
            Event::UserEvent(UserEvent::ToggleAutostart) => {
                let enabled = !autostart::enabled();
                if let Ok(exe) = std::env::current_exe() {
                    let _ = autostart::set(enabled, &exe);
                }
                autostart_item.set_checked(autostart::enabled());
            }
            Event::UserEvent(UserEvent::OpenLog) => {
                let log = settings_file.with_file_name("log.txt");
                if let Some(dir) = log.parent() {
                    let _ = fs::create_dir_all(dir);
                }
                let _ = Command::new("notepad.exe").arg(log).spawn();
            }
            Event::UserEvent(UserEvent::CheckUpdates(manual)) => {
                update_status.set_text("Checking for updates...");
                update_status.set_enabled(false);
                let check_proxy = proxy.clone();
                thread::spawn(move || {
                    let res = updater::check_for_updates();
                    let _ = check_proxy.send_event(UserEvent::UpdateResult {
                        result: res,
                        manual,
                    });
                });
            }
            Event::UserEvent(UserEvent::UpdateResult { result, manual }) => {
                match result {
                    Ok(Some(info)) => {
                        available_update = Some(info.clone());
                        update_status.set_text(&format!("★ New version available: {} (Click to download)", info.tag));
                        update_status.set_enabled(true);
                        if let Some(current) = tray.as_ref() {
                            let _ = current.set_tooltip(Some(format_tooltip(last, &available_update)));
                        }
                        let info_box = info.clone();
                        thread::spawn(move || {
                            if updater::show_update_prompt(&info_box.tag) {
                                updater::open_url(&info_box.url);
                            }
                        });
                    }
                    Ok(None) => {
                        available_update = None;
                        let cur = env!("CARGO_PKG_VERSION");
                        update_status.set_text(&format!("Up to date (v{cur})"));
                        update_status.set_enabled(false);
                        if let Some(current) = tray.as_ref() {
                            let _ = current.set_tooltip(Some(format_tooltip(last, &available_update)));
                        }
                        if manual {
                            thread::spawn(move || {
                                updater::show_up_to_date_message(cur);
                            });
                        }
                    }
                    Err(err) => {
                        update_status.set_text("Update check failed");
                        update_status.set_enabled(false);
                        if manual {
                            thread::spawn(move || {
                                updater::show_error_message(&err);
                            });
                        }
                    }
                }
            }
            Event::UserEvent(UserEvent::OpenUpdateUrl) => {
                if let Some(info) = &available_update {
                    updater::open_url(&info.url);
                } else {
                    updater::open_url("https://github.com/GuacamoleGH/HyperX-Battery-Tray/releases/latest");
                }
            }
            Event::UserEvent(UserEvent::Quit) => {
                tray.take();
                *flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Reading(result)) => {
                let state = match result {
                    Ok(value) => {
                        last = Some(value.percent);
                        failures = 0;
                        format!("Battery: {}%", value.percent)
                    }
                    Err(error) => {
                        failures += 1;
                        if failures > 3 {
                            last = None;
                        }
                        error.to_string()
                    }
                };
                status.set_text(&state);
                if let Some(current) = tray.as_ref() {
                    let _ = current.set_tooltip(Some(format_tooltip(last, &available_update)));
                    let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold);
                    let _ = current.set_icon(Some(
                        tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"),
                    ));
                }
            }
            _ => {}
        }
    });
}
