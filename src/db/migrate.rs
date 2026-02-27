use sqlx::{migrate::MigrateError, PgPool};

pub async fn run(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}