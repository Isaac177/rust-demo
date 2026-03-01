use axum::{routing::{get, post, patch, delete}, Router};
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
        .route("/auth/register", post(crate::features::auth::handler::register))
        .route("/auth/login", post(crate::features::auth::handler::login))
        .route("/news", get(crate::features::news::handler::get_all_news_handler))
        .route("/news/{id}", get(crate::features::news::handler::get_news_by_id))
        .route("/news/{id}", patch(crate::features::news::handler::update_news_handler))
        .route("/news/{id}", delete(crate::features::news::handler::delete_news_handler))
        .route("/news/create", post(crate::features::news::handler::create_news_handler))
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