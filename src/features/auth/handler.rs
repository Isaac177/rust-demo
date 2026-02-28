use crate::app::state::AppState;
use crate::features::auth::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::features::auth::service;
use crate::http::error::ApiResult;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), crate::http::error::AppError> {
    let response = service::register(&state.db_pool, input, &state.settings.auth.jwt_secret).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    let response = service::login(&state.db_pool, input, &state.settings.auth.jwt_secret).await?;

    Ok(Json(response))
}