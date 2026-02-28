use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct UserAuthRecord {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub password_hash: String,
    pub is_active: bool,
}

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserAuthRecord>, sqlx::Error> {
    sqlx::query_as!(
        UserAuthRecord,
        r#"
SELECT
    id,
    email,
    full_name,
    password_hash,
    is_active
FROM users
WHERE email = $1
"#,
        email
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    full_name: &str,
    password_hash: &str,
) -> Result<UserAuthRecord, sqlx::Error> {
    sqlx::query_as!(
        UserAuthRecord,
        r#"
INSERT INTO users (email, full_name, password_hash)
VALUES ($1, $2, $3)
RETURNING id, email, full_name, password_hash, is_active
"#,
        email, full_name, password_hash
    )
        .fetch_one(pool)
        .await
}

pub async fn find_user_by_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<Option<UserAuthRecord>, sqlx::Error> {
    sqlx::query_as!(
        UserAuthRecord,
        r#"
SELECT id, email, full_name, password_hash, is_active
FROM users 
WHERE id = $1
"#,
        user_id
    )
        .fetch_optional(pool)
        .await
}