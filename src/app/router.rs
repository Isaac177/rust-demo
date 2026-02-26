use axum::{routing::get, Router};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer}
};
use tracing::Level;

use super::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(crate::features::health::handler::live))
        .route("/health/ready", get(crate::features::health::handler::ready))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(state)
}