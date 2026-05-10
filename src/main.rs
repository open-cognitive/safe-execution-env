//! # Open-Cognitive: Safe Execution Environment - Bus Worker

mod sandbox;

use sandbox::WasmSandbox;
use open_cognitive_protocol::{CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Safe Execution (BUS WORKER) Başlatılıyor ===");
    
    let sandbox = WasmSandbox::new().expect("Sandbox başlatılamadı!");
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Sandbox Bellek Otobüsü dinleniyor: /tmp/cog.bus");

    // Simülasyon için sabit WASM kodu
    let tool_code = r#"
    (module
      (func $execute (param $input i32) (result i32)
        local.get $input
        i32.const 10
        i32.mul
      )
      (export "execute" (func $execute))
    )
    "#;

    loop {
        let signal = bus.read_signal();

        if signal.command_type == CMD_EXECUTE_TOOL {
            println!("\n[SANDBOX] 'Act' (Eylem) emri alındı! İzole araç çalıştırılıyor...");
            
            match sandbox.execute_tool(tool_code, 5) {
                Ok(res) => println!("[BAŞARILI] Araç Çıktısı: {}", res),
                Err(e) => eprintln!("[HATA] Sandbox yürütmeyi durdurdu: {}", e),
            }

            // İşi bitirince belleği temizle (ACK)
            let mut ack_signal = signal;
            ack_signal.command_type = CMD_IDLE;
            bus.write_signal(&ack_signal);
            println!("[SİSTEM] Görev bitti, otobüs temizlendi.");
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}