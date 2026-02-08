
use blake3::Hasher;
use serde_json::Value;
use tdln_canon::json_atomic_stringify;

/// Compute CID as `b3:<hex>` over JSON Atomic bytes.
pub fn cid_from_json(value: &Value) -> String {
    let s = json_atomic_stringify(value);
    let mut hasher = Hasher::new();
    hasher.update(s.as_bytes());
    let h = hasher.finalize();
    format!("b3:{:x}", h)
}

/// Compute CID as `b3:<hex>` over raw bytes.
pub fn cid_from_bytes(bytes: &[u8]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(bytes);
    let h = hasher.finalize();
    format!("b3:{:x}", h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn cid_stable() {
        let a = json!({"b":1,"a":2});
        let b = json!({"a":2,"b":1});
        assert_eq!(cid_from_json(&a), cid_from_json(&b));
    }
}
