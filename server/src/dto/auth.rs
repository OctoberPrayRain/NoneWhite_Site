use serde::{Deserialize, Serialize};

use crate::dto::users::UserResponse;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokenResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserResponse,
}
