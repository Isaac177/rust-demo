use axum::extract::State;
use crate::{
    app::state::AppState,
    db::health::ping,
    http::{
        error::{ApiResult, AppError},
        response::{ok, ApiJson}
    }
};

pub async fn live() -> ApiResult<ApiJson<&'static str>>{
    Ok(ok("Your app is live"))
}

pub async fn ready(State(state): State<AppState>) -> ApiResult<ApiJson<&'static str>> {
    ping(&state.db_pool).await.map_err(|err| {
        tracing::error!(db_error = %err, "Database readiness check failed");
        AppError::Internal("Database connection is not ready".to_string())
    })?;

    Ok(ok("Ready"))
}
