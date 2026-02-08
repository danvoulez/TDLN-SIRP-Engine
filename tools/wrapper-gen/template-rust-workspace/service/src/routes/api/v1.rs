use axum::{routing::post, Router};
use std::sync::Arc;
use crate::state::AppState;

use super::engine::run;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/run", post(run))
        .with_state(state)
}
