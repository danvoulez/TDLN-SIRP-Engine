use anyhow::Result;
use serde_json::Value;

pub async fn pre() -> Result<()> { Ok(()) }
pub async fn post(_card: &Value) -> Result<()> { Ok(()) }
