use crate::app::state::AppState;
use crate::features::auth::current_user::CurrentUser;
use crate::features::news::dto::{NewsInput, NewsResponse};
use crate::features::news::service::{create_post, get_all_posts, get_post_by_id};
use crate::http::error::{ApiResult, AppError};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

pub async fn create_news_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(input): Json<NewsInput>,
) -> Result<(StatusCode, Json<NewsResponse>), crate::http::error::AppError> {
    let response = create_post(&state.db_pool, current_user.id, input).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_all_news_handler(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<NewsResponse>>> {
    let response = get_all_posts(&state.db_pool).await?;

    Ok(Json(response))
}

pub async fn get_news_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<Json<NewsResponse>, AppError> {
    let response = get_post_by_id(&state.db_pool, id).await?;

    Ok(Json(response))
}