use sqlx::types::chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct CommentRow {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub game_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct CommentOwnerRow {
    pub user_id: i64,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ParentCommentRow {
    pub game_id: i64,
}
