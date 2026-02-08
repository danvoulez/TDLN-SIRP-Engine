
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineRegistryEntry {
    pub kind: String,             // "engine.registry.entry.v1"
    pub id: String,               // ULID or similar
    pub name: String,
    pub version: String,
    pub cid: String,              // content address for the artifact
    pub meta: serde_json::Value,  // free-form metadata
}
