use hyperx_battery::headset::{self, is_target, PIDS, VID};
use hidapi::HidApi;
fn hex(data:&[u8])->String { data.iter().take(16).map(|b|format!("{b:02X}")).collect::<Vec<_>>().join(" ") }
fn main() {
    let api=match HidApi::new(){Ok(a)=>a,Err(e)=>{eprintln!("No se pudo iniciar HID: {e}");return}};
    println!("Interfaces HID con VID {VID:04X}:");
    for d in api.device_list().filter(|d|d.vendor_id()==VID){let mark=if PIDS.contains(&d.product_id())&&is_target(d){" <- se usará"}else{""};println!("  PID {:04X} interfaz {} usage_page {:#06x} usage {:#06x}{}",d.product_id(),d.interface_number(),d.usage_page(),d.usage(),mark);}
    match headset::read_battery(){Ok(v)=>{println!("BATERÍA = {}%",v.percent);if let Some(mv)=v.millivolts{println!("Tensión: {:.3} V",mv as f32/1000.0)}},Err(e)=>eprintln!("{e}")}
    let _=hex(&[]); // Conserva el formato de hexdump para futuras capturas de diagnóstico.
}
