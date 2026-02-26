use axum::{routing::get, Router};

use super::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(|| async { "healthy" }))
        .route("/health/ready", get(crate::features::health::handler::ready))
        .with_state(state)
}