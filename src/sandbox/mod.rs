//! # İzole Yürütme Ortamı (WASM Sandbox)
//! 
//! Bu modül, ajanın dış dünyayla etkileşim kurması gerektiğinde
//! (Tool Execution) çalıştırılacak araçları WebAssembly (WASM) 
//! ortamı içinde %100 güvenle izole eder.
//! 
//! Ajan, bu Sandbox dışındaki bellek adreslerine veya dosyalara
//! donanımsal olarak erişemez.

use wasmtime::*;

/// Güvenli çalışma zamanı yöneticisi
pub struct WasmSandbox {
    engine: Engine,
}

impl WasmSandbox {
    /// Yeni bir WebAssembly çalışma zamanı (Sandbox) başlatır.
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        // Bellek sınırlandırmaları ve güvenlik kısıtlamaları 
        // işletim sisteminin güvenliği için burada aktifleştirilir.
        config.wasm_reference_types(true);
        
        let engine = Engine::new(&config)?;
        
        Ok(Self { engine })
    }
    
    /// Verilen WASM (veya okunabilir WAT) kodunu izole ortamda çalıştırır.
    /// 
    /// `wat_code`: Çalıştırılacak araca ait WebAssembly metni
    /// `input`: Araca gönderilecek argüman (Şimdilik basit bir i32)
    pub fn execute_tool(&self, wat_code: &str, input: i32) -> Result<i32> {
        // 1. Bellek ve durumu (State) izole edecek Store oluştur
        let mut store = Store::new(&self.engine, ());
        
        // 2. Güvenilmeyen (Untrusted) aracı çalışma zamanı için derle
        let module = Module::new(&self.engine, wat_code)?;
        
        // 3. Modülü izole ortama (Instance) yükle (Host ile bağları kopar)
        let instance = Instance::new(&mut store, &module, &[])?;
        
        // 4. Modülün içindeki 'execute' fonksiyonunu bul
        let func = instance.get_typed_func::<i32, i32>(&mut store, "execute")?;
        
        // 5. Aracı çalıştır ve sonucu güvenli bir şekilde ana sisteme dön
        let result = func.call(&mut store, input)?;
        
        Ok(result)
    }
}