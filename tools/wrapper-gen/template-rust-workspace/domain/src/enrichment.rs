use serde_json::Value;

pub fn add_badges(card: &mut Value) {
    if let Some(obj) = card.as_object_mut() {
        obj.insert("badge".into(), Value::String("verified".into()));
    }
}
