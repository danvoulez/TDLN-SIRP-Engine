
use serde_json::{Map, Value};

/// JSON Atomic: deterministic, UTF-8, sorted keys, no insignificant whitespace.
/// Conservative implementation using serde_json Value normalization.
pub fn json_atomic_stringify(value: &Value) -> String {
    let v = normalize(value);
    // Use compact serializer with stable map ordering (we sorted recursively)
    serde_json::to_string(&v).expect("serialize")
}

fn normalize(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut out = Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort(); // lexicographic UTF-8 order
            for k in keys {
                out.insert(k.clone(), normalize(&map[k]));
            }
            Value::Object(out)
        }
        Value::Array(arr) => {
            // Arrays preserve order
            Value::Array(arr.iter().map(normalize).collect())
        }
        // Numbers/strings/bools/null are already canonical under serde_json
        _ => v.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn canon_sorts_keys() {
        let v = json!({"b":1,"a":{"y":2,"x":1}});
        let s = json_atomic_stringify(&v);
        assert_eq!(s, r#"{"a":{"x":1,"y":2},"b":1}"#);
    }
}
