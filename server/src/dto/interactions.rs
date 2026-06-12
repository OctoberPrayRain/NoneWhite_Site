use serde::{Deserialize, Serialize};

use crate::models::interaction::CommentRow;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct InteractionListParams {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub avatar_url: Option<String>,
    pub game_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentListResponse {
    pub list: Vec<CommentResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeStatusResponse {
    pub liked: bool,
    pub likes_count: i32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteStatusResponse {
    pub favorited: bool,
    pub favorites_count: i32,
}

impl From<CommentRow> for CommentResponse {
    fn from(comment: CommentRow) -> Self {
        Self {
            id: comment.id,
            user_id: comment.user_id,
            username: comment.username,
            avatar_url: comment.avatar_url,
            game_id: comment.game_id,
            content: comment.content,
            parent_id: comment.parent_id,
            created_at: comment.created_at.to_rfc3339(),
        }
    }
}
