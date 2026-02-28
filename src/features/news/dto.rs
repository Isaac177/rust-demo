use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewsInput {
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Debug, Serialize)]
pub struct NewsResponse {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}