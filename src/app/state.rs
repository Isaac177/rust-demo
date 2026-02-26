use sqlx::PgPool;

use crate::config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub db_pool: PgPool,
}