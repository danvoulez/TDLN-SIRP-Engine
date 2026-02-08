
// Deterministic ABI wrapper for Engine v12+
// Exports: memory (implicit), alloc, dealloc, run(ptr,len)->(ptr,len)
use serde_json::Value as Json;

#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let mut buf = Vec::<u8>::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr as i32
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {
    unsafe { let _ = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize); }
}

#[no_mangle]
pub extern "C" fn run(ptr: i32, len: i32) -> (i32, i32) {
    // 1) read input bytes (canonical JSON enforced by host, but we keep strictness)
    let input = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let j: Json = serde_json::from_slice(input).unwrap_or(Json::Null);

    // 2) call deterministic logic (pure function, no I/O)
    let out = logic::execute(j);

    // 3) write result bytes (host will canonize again)
    let bytes = serde_json::to_vec(&out).unwrap();
    let out_ptr = alloc(bytes.len() as i32);
    unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, bytes.len()).copy_from_slice(&bytes);
    }
    (out_ptr, bytes.len() as i32)
}

mod logic {
    use serde_json::{json, Value as Json};

    /// Deterministic transform: merge with a minimal stamp and echo input.
    /// Replace this function with your domain logic (must stay pure/deterministic).
    pub fn execute(mut input: Json) -> Json {
        // avoid time/rand/env here; the host ensures determinism and re‑canonizes output
        match &mut input {
            Json::Object(m) => {
                // Stamp: version & echo flag (constants → deterministic)
                m.insert("engine_profile".into(), json!("deterministic@v1"));
                m.insert("echo".into(), Json::Bool(true));
            }
            _ => { /* keep as is */ }
        }
        input
    }
}
