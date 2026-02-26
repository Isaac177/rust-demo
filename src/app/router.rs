use axum::{routing::get, Router};

use super::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(|| async { "healthy" }))
        .with_state(state)
}