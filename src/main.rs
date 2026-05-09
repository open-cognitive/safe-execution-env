//! # Open-Cognitive: Safe Execution Environment (Sistem 3)
//! 
//! Bilişsel İşletim Sisteminin "Eylem" (Act) katmanı.
//! Mantık çekirdeğinin kararları burada donanımsal olarak kısıtlanmış
//! sanal makinelerde (WASM) fiziksel sonuçlara dönüştürülür.

mod sandbox;

use sandbox::WasmSandbox;

fn main() {
    println!("=== Open-Cognitive Safe Execution Environment Başlatılıyor ===");
    println!("WASM Sandbox Güvenlik Duvarı Aktif...\n");

    // Sandbox'ı ayağa kaldır
    let sandbox = WasmSandbox::new().expect("Sandbox başlatılamadı!");

    // SİMÜLASYON: Logic Gate Core 'Act' durumunda ve bir matematik aracı çalıştırmak istiyor.
    // Dışarıdan yüklenen (güvenilmeyen) bir WebAssembly aracı (Tool).
    // (Aşağıdaki kod, girdiyi alıp 2 ile çarpan saf WebAssembly [WAT] komutlarıdır)
    let untrusted_tool_code = r#"
    (module
      (func $execute (param $input i32) (result i32)
        ;; Ajanın kullanacağı hesaplama aracı (Tool)
        ;; Gelen sayıyı al (local.get) ve 2 sabitiyle (i32.const 2) çarp (i32.mul)
        local.get $input
        i32.const 2
        i32.mul
      )
      (export "execute" (func $execute))
    )
    "#;

    println!("[SİSTEM] Mantık Çekirdeği bir Eylem (Act) komutu gönderdi.");
    println!("[SANDBOX] 'Hesaplama Aracı' güvenli ortama yükleniyor...");

    let tool_input = 21; // Araca gönderilen görev verisi
    
    // Aracı Sandbox içinde çalıştır
    match sandbox.execute_tool(untrusted_tool_code, tool_input) {
        Ok(result) => {
            println!("\n[BAŞARILI] Araç izole ortamda çalıştırıldı!");
            println!("Girdi: {} -> Araç Çıktısı: {}", tool_input, result);
            println!("(Çıktı, 'Reflect' yapması için Shared Memory üzerinden Logic Gate'e gönderilecek)");
        },
        Err(e) => {
            eprintln!("\n[GÜVENLİK İHLALİ VEYA HATA] Sandbox yürütmeyi durdurdu: {}", e);
        }
    }
}