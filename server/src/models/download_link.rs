use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct DownloadLinkRow {
    pub id: i64,
    pub game_id: i64,
    pub platform: String,
    pub url: String,
    pub extract_code: Option<String>,
    pub password: Option<String>,
    pub file_size: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
