use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct NewsRecord {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_news(
    pool: &PgPool,
    user_id: i64,
    title: &str,
    body: &str,
    published: bool
) -> Result<NewsRecord, sqlx::Error> {
    sqlx::query_as!(
        NewsRecord,
        r#"
INSERT INTO posts (user_id, title, body, published)
VALUES ($1, $2, $3, $4)
RETURNING id, user_id, title, body, published, created_at, updated_at
   "#,
        user_id, title, body, published
    ).fetch_one(pool).await
}

pub async fn get_news_by_id(
    pool: &PgPool,
    id: i64,
) -> Result<Option<NewsRecord>, sqlx::Error> {
    sqlx::query_as!(
        NewsRecord,
        r#"
SELECT id, title, body, user_id, created_at, published, updated_at
FROM posts WHERE id = $1
"#, id,
    ).fetch_optional(pool).await
}

pub async fn get_all_news(
    pool: &PgPool,
) -> Result<Vec<NewsRecord>, sqlx::Error> {
    sqlx::query_as!(
        NewsRecord,
        r#"
SELECT id, user_id, title, body, published, created_at, updated_at FROM posts
"#,
    ).fetch_all(pool).await
}

pub async fn update_news(
    pool: &PgPool,
    id: i64,
    user_id: i64,
    title: &str,
    body: &str,
    published: bool,
) -> Result<Option<NewsRecord>, sqlx::Error> {
    sqlx::query_as!(
        NewsRecord,
        r#"
UPDATE posts
SET
    title = $1,
    body = $2,
    published = $3,
    updated_at = NOW()
WHERE id = $4 AND user_id = $5
RETURNING id, user_id, title, body, published, created_at, updated_at
"#,
        title,
        body,
        published,
        id,
        user_id,
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete_news(
    pool: &PgPool,
    id: i64,
    user_id: i64
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1 AND user_id = $2", id, user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}