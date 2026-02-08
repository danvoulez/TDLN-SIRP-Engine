use serde_json::json;

#[test]
fn json_atomic_golden() {
    let v = json!({"z":2,"a":1,"arr":[{"b":2,"a":1},3]});
    let bytes = engine_core::json_atomic::to_json_atomic_bytes(&v).unwrap();
    let s = std::str::from_utf8(&bytes).unwrap();
    assert_eq!(s, r#"{\"a\":1,\"arr\":[{\"a\":1,\"b\":2},3],\"z\":2}"#);
}
