
use anyhow::{anyhow, Result, bail};
use serde_json::Value as Json;
use wasmtime::{Engine, Module, Store, Config, Linker, TypedFunc};
use wasmparser::{Parser, Payload};

#[derive(Clone, Debug)]
pub struct ExecConfig { pub fuel_limit: u64, pub memory_limit_bytes: usize, pub allow_imports: bool }
impl Default for ExecConfig { fn default()->Self{ Self{ fuel_limit: 50_000_000, memory_limit_bytes: 33554432, allow_imports:false } }}

pub struct WasmExecutor{ engine: Engine, cfg: ExecConfig }
impl WasmExecutor{
    pub fn new(cfg: ExecConfig)->Result<Self>{
        let mut c = Config::new();
        c.consume_fuel(true).cranelift_nan_canonicalization(true).wasm_multi_value(true).wasm_simd(false).wasm_threads(false);
        let engine = Engine::new(&c)?; Ok(Self{engine,cfg})
    }
    fn validate(&self, bytes:&[u8])->Result<()>{
        if !self.cfg.allow_imports {
            for p in Parser::new(0).parse_all(bytes){
                if let Payload::ImportSection(_) = p? { bail!("imports not allowed in deterministic mode"); }
            }
        }
        Ok(())
    }
    pub fn exec(&self, unit:&[u8], input:&Json)->Result<Vec<u8>>{
        self.validate(unit)?;
        let in_canon = canon(input); let in_bytes = serde_json::to_vec(&in_canon)?;
        let module = Module::new(&self.engine, unit)?;
        let mut store = Store::new(&self.engine,());
        store.add_fuel(self.cfg.fuel_limit)?;
        let linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance.get_memory(&mut store, "memory").ok_or_else(|| anyhow!("export memory required"))?;
        let alloc: TypedFunc<i32,i32> = instance.get_typed_func(&mut store, "alloc").map_err(|_| anyhow!("export alloc required"))?;
        let dealloc: TypedFunc<(i32,i32),()> = instance.get_typed_func(&mut store, "dealloc").map_err(|_| anyhow!("export dealloc required"))?;
        let run: TypedFunc<(i32,i32),(i32,i32)> = instance.get_typed_func(&mut store, "run").map_err(|_| anyhow!("export run required"))?;
        if memory.data_size(&store) > self.cfg.memory_limit_bytes { bail!("guest memory exceeds limit"); }
        if in_bytes.len() > self.cfg.memory_limit_bytes { bail!("input exceeds limit"); }
        let in_ptr = alloc.call(&mut store, in_bytes.len() as i32)?;
        let data = memory.data_mut(&mut store);
        let s = in_ptr as usize; let e = s + in_bytes.len(); if e > data.len(){ bail!("bad alloc OOB"); }
        data[s:e].copy_from_slice(&in_bytes);
        let (out_ptr, out_len) = run.call(&mut store, (in_ptr, in_bytes.len() as i32))?;
        let data2 = memory.data(&store);
        let s2 = out_ptr as usize; let e2 = s2 + (out_len as usize); if e2 > data2.len(){ bail!("guest OOB"); }
        let out = data2[s2:e2].to_vec();
        let _ = dealloc.call(&mut store, (in_ptr, in_bytes.len() as i32));
        let _ = dealloc.call(&mut store, (out_ptr, out_len));
        let j: Json = serde_json::from_slice(&out)?;
        let out_canon = canon(&j);
        Ok(serde_json::to_vec(&out_canon)?)
    }
}

fn canon(v:&Json)->Json{
    match v{
        Json::Object(m)=>{
            let mut ks:Vec<_>=m.keys().collect(); ks.sort();
            let mut new = serde_json::Map::new();
            for k in ks{ new.insert(k.clone(), canon(&m[k])); }
            Json::Object(new)
        },
        Json::Array(a)=> Json::Array(a.iter().map(canon).collect()),
        _=> v.clone()
    }
}
