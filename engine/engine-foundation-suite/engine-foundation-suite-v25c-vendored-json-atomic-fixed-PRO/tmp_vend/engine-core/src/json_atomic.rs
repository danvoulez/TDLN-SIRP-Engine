use anyhow::{Result, Context};
use serde::Serialize;
use serde_json::{Value, Map};
use std::collections::BTreeMap;

/// JSON✯Atomic canonicalization (G1 Paper II semantics as implemented in g1/tdln-core):
/// - Objects: keys sorted lexicographically by UTF-8 bytes (Rust String Ord via BTreeMap)
/// - Arrays: element-wise canonicalization
/// - Scalars: unchanged
pub fn canonize(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut sorted: BTreeMap<String, Value> = BTreeMap::new();
            for (k, val) in m.iter() {
                sorted.insert(k.clone(), canonize(val));
            }
            let mut out = Map::new();
            for (k, val) in sorted.into_iter() {
                out.insert(k, val);
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.iter().map(canonize).collect()),
        _ => v.clone(),
    }
}

/// Serialize -> JSON Value -> canonize -> minified JSON bytes (UTF-8)
pub fn to_json_atomic_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let v = serde_json::to_value(value).context("to_value")?;
    let canon = canonize(&v);
    serde_json::to_vec(&canon).context("to_vec")
}

/// Compute CID: b3:<hex(blake3(json_atomic_bytes))>
pub fn compute_cid<T: Serialize>(value: &T) -> Result<String> {
    let bytes = to_json_atomic_bytes(value)?;
    let h = blake3::hash(&bytes);
    Ok(format!("b3:{}", h.to_hex()))
}

/// Compute DID: did:llf:<cid>
pub fn compute_did<T: Serialize>(value: &T) -> Result<String> {
    let cid = compute_cid(value)?;
    Ok(format!("did:llf:{}", cid))
}
