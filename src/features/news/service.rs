use crate::features::news::dto::{NewsInput, NewsResponse};
use crate::features::news::repository::{create_news, delete_news, get_all_news, get_news_by_id, update_news, NewsRecord};
use crate::http::error::AppError;
use sqlx::PgPool;

pub async fn create_post(pool: &PgPool, user_id: i64, input: NewsInput) -> Result<NewsResponse, AppError> {
    validate_news_input(&input)?;

    let news = create_news(
        pool,
        user_id,
        &input.title.as_str(),
        &input.body.as_str(),
        input.published,
    ).await.map_err(internal_error)?;

    Ok(map_news_record(news))
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<NewsResponse>, AppError> {
    let news = get_all_news(pool).await.map_err(internal_error)?;

    Ok(news.into_iter().map(map_news_record).collect())
}

pub async fn get_post_by_id(pool: &PgPool, id: i64) -> Result<NewsResponse, AppError> {
    let news = get_news_by_id(pool, id)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| AppError::Validation("news not found".to_string()))?;

    Ok(map_news_record(news))
}

pub async fn update_post(pool: &PgPool, id: i64, user_id: i64, input: NewsInput) -> Result<NewsResponse, AppError> {
    if id <= 0 {
        return Err(AppError::Validation("invalid post id".to_string()));
    }

    validate_news_input(&input)?;

    let news = update_news(pool, id, user_id, input.title.as_str(), input.body.as_str(), input.published)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| AppError::Validation("news not found".to_string()))?;

    Ok(map_news_record(news))
}

pub async fn delete_post(
    pool: &PgPool,
    id: i64,
    user_id: i64
) -> Result<(), AppError> {
    if id <= 0 {
        return Err(AppError::Validation("invalid post id".to_string()));
    }

    let deleted = delete_news(pool, id, user_id)
        .await
        .map_err(internal_error)?;

    if !deleted {
        return Err(AppError::Validation("news not found".to_string()));
    }

    Ok(())
}

fn validate_news_input(input: &NewsInput) -> Result<(), AppError> {

    if input.title.trim().is_empty() {
        return Err(AppError::Validation("Title is empty".to_string()));
    }

    if input.body.trim().is_empty() {
        return Err(AppError::Validation("Body is empty".to_string()));
    }

    Ok(())
}

fn internal_error(err: sqlx::Error) -> AppError {
    AppError::Internal(format!("sqlx error: {err}"))
}

fn map_news_record(news: NewsRecord) -> NewsResponse {
    NewsResponse {
        id: news.id,
        user_id: news.user_id,
        title: news.title,
        body: news.body,
        published: news.published,
        created_at: news.created_at,
        updated_at: news.updated_at
    }
}
