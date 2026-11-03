use hyperx_battery::{headset, icons, settings::Settings};
use std::{sync::mpsc,thread,time::Duration};
use tao::{event::{Event,StartCause},event_loop::{ControlFlow,EventLoopBuilder}};
use tray_icon::{TrayIconBuilder,TrayIcon};

#[derive(Clone,Debug)] enum UserEvent{ Reading(Result<headset::BatteryReading,headset::ReadError>) }
pub fn run(){
 let event_loop=EventLoopBuilder::<UserEvent>::with_user_event().build(); let proxy=event_loop.create_proxy();
 let (tx,rx)=mpsc::channel::<()>(); thread::spawn(move||loop{let r=headset::read_battery();let _=proxy.send_event(UserEvent::Reading(r));if rx.recv_timeout(Duration::from_secs(60)).is_ok(){break}});
 let mut tray:Option<TrayIcon>=None; let mut last:Option<u8>=None; let mut failures=0u8; let settings=Settings::default();
 event_loop.run(move|event,_,flow|{*flow=ControlFlow::Wait;match event{
   Event::NewEvents(StartCause::Init)=>{let icon=icons::make_icon(16,settings.display_mode,last,settings.low_threshold);let ti=tray_icon::Icon::from_rgba(icon.rgba,icon.width,icon.height).unwrap();tray=Some(TrayIconBuilder::new().with_tooltip("HyperX Cloud III S: no data").with_icon(ti).build().unwrap());}
   Event::UserEvent(UserEvent::Reading(result))=>{match result{Ok(v)=>{last=Some(v.percent);failures=0;if let Some(t)=tray.as_ref(){t.set_tooltip(Some(&format!("HyperX Cloud III S: {}%",v.percent))).ok();}},Err(_)=>{failures+=1;if failures>3{last=None;}}};if let Some(t)=tray.as_ref(){let i=icons::make_icon(16,settings.display_mode,last,settings.low_threshold);t.set_icon(Some(tray_icon::Icon::from_rgba(i.rgba,i.width,i.height).unwrap()));}}
   _=>{}
 }}); drop(tx);
}
