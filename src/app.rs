use hyperx_battery::{headset, icons, settings::Settings};
use std::thread;
use tao::{event::{Event, StartCause}, event_loop::{ControlFlow, EventLoopBuilder}};
use tray_icon::{menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, TrayIcon, TrayIconBuilder};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSMICON};

#[derive(Clone, Debug)]
enum UserEvent { Reading(Result<headset::BatteryReading, headset::ReadError>), Refresh, Quit }

fn tray_size() -> u32 { unsafe { GetSystemMetrics(SM_CXSMICON).max(32) as u32 } }

pub fn run() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let menu = Menu::new();
    let status = MenuItem::with_id("status", "Status: no data", false, None);
    let refresh = MenuItem::with_id("refresh", "Refresh now", true, None);
    let quit = MenuItem::with_id("quit", "Exit", true, None);
    menu.append_items(&[&status, &refresh, &PredefinedMenuItem::separator(), &quit]).expect("menu");
    let menu_proxy = proxy.clone();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| { let _ = match event.id.as_ref() { "refresh" => menu_proxy.send_event(UserEvent::Refresh), "quit" => menu_proxy.send_event(UserEvent::Quit), _ => Ok(()) }; }));
    let initial_proxy = proxy.clone();
    thread::spawn(move || { let _ = initial_proxy.send_event(UserEvent::Reading(headset::read_battery())); });
    let mut menu = Some(menu);
    let mut tray: Option<TrayIcon> = None;
    let mut last: Option<u8> = None;
    let mut failures = 0u8;
    let settings = Settings::default();
    let icon_size = tray_size();
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => { let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold); let icon = tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"); tray = Some(TrayIconBuilder::new().with_menu(Box::new(menu.take().expect("menu"))).with_tooltip("HyperX Cloud III S: no data").with_icon(icon).build().expect("tray icon")); }
            Event::UserEvent(UserEvent::Refresh) => { let reader = proxy.clone(); thread::spawn(move || { let _ = reader.send_event(UserEvent::Reading(headset::read_battery())); }); }
            Event::UserEvent(UserEvent::Quit) => { tray.take(); *flow = ControlFlow::Exit; }
            Event::UserEvent(UserEvent::Reading(result)) => { let state = match result { Ok(value) => { last = Some(value.percent); failures = 0; format!("Battery: {}%", value.percent) }, Err(error) => { failures += 1; if failures > 3 { last = None; } error.to_string() } }; status.set_text(&state); if let Some(current) = tray.as_ref() { let tooltip = last.map(|p| format!("HyperX Cloud III S: {p}%")).unwrap_or_else(|| "HyperX Cloud III S: no data".to_string()); let _ = current.set_tooltip(Some(tooltip)); let pixels = icons::make_icon(icon_size, settings.display_mode, last, settings.low_threshold); let _ = current.set_icon(Some(tray_icon::Icon::from_rgba(pixels.rgba, pixels.width, pixels.height).expect("icon"))); } }
            _ => {}
        }
    });
}
