//! # Open-Cognitive: Safe Execution Environment - Dynamic Worker

mod sandbox;

use sandbox::WasmSandbox;
use open_cognitive_protocol::{CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Safe Execution (DYNAMIC WORKER) Başlatılıyor ===");
    
    let sandbox = WasmSandbox::new().expect("Sandbox başlatılamadı!");
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;

    loop {
        let signal = bus.read_signal();

        if signal.command_type == CMD_EXECUTE_TOOL {
            let dynamic_input = signal.payload[0] as i32;
            println!("\n[SANDBOX] Görev alındı. İşlenecek Girdi: {}", dynamic_input);
            
            let mut ack_signal = signal;
            
            // YENİ WASM TARGET İSMİNE GÖRE YOLU GÜNCELLEDİK (wasip1)
            let wasm_path = "../tool-wasi-sdk/target/wasm32-wasip1/debug/tool_wasi_sdk.wasm";

            match sandbox.execute_wasm_file(wasm_path, dynamic_input) {
                Ok(res) => {
                    println!("[BAŞARILI] Harici WASM Aracı Çalıştırıldı!");
                    println!("[BAŞARILI] Araç Çıktısı ({} ^ 2): {}", dynamic_input, res);
                    ack_signal.payload[0] = res as u8; 
                },
                Err(e) => eprintln!("[HATA] Sandbox yürütmeyi durdurdu: {}", e),
            }

            ack_signal.command_type = CMD_IDLE;
            bus.write_signal(&ack_signal);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}