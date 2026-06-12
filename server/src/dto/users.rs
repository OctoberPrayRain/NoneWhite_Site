use serde::{Deserialize, Serialize};

use crate::models::user::UserRow;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserProfileRequest {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUploadResponse {
    pub avatar_url: String,
}

impl From<UserRow> for UserResponse {
    fn from(user: UserRow) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            role: user.role,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}
