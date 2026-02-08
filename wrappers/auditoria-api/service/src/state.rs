use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct AppState {
    pub cartridge_rev: Arc<RwLock<u64>>,
}
