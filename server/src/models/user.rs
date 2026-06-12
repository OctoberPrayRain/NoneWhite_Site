use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
