//! # İzole Yürütme Ortamı (WASM Sandbox)
//! 
//! Bu modül, WebAssembly (WASM) ortamı içinde araçları güvenle çalıştırır.

use wasmtime::*;
use anyhow::Result;

pub struct WasmSandbox {
    engine: Engine,
}

impl WasmSandbox {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_reference_types(true);
        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }
    
    /// Derlenmiş bir .wasm dosyasını diskten okuyup izole ortamda çalıştırır.
    pub fn execute_wasm_file(&self, file_path: &str, input: i32) -> Result<i32> {
        let wasm_bytes = std::fs::read(file_path)
            .map_err(|e| anyhow::anyhow!("WASM dosyası bulunamadı ({}): {}", file_path, e))?;
        
        let mut store = Store::new(&self.engine, ());
        let module = Module::new(&self.engine, &wasm_bytes)?;
        let instance = Instance::new(&mut store, &module, &[])?;
        let func = instance.get_typed_func::<i32, i32>(&mut store, "execute")?;
        
        let result = func.call(&mut store, input)?;
        Ok(result)
    }
}