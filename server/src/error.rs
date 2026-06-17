use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::Value;

use crate::response::ApiResponse;

#[derive(Clone, Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub code: u16,
    pub message: &'static str,
}

impl AppError {
    pub const fn new(status: StatusCode, code: u16, message: &'static str) -> Self {
        Self {
            status,
            code,
            message,
        }
    }

    pub const fn username_invalid() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40001, "Username is invalid")
    }

    pub const fn email_invalid() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40002, "Email is invalid")
    }

    pub const fn password_too_short() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40003, "Password is too short")
    }

    pub const fn password_required() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40004, "Password is required")
    }

    pub const fn avatar_url_update_forbidden() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            40005,
            "Avatar URL cannot be updated directly",
        )
    }

    pub const fn avatar_file_required() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40006, "Avatar file is required")
    }

    pub const fn avatar_file_type_not_allowed() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            40007,
            "Avatar file type is not allowed",
        )
    }

    pub const fn avatar_file_too_large() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40008, "Avatar file is too large")
    }

    pub const fn comment_content_required() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            40009,
            "Comment content is required",
        )
    }

    pub const fn comment_content_too_long() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            40010,
            "Comment content is too long",
        )
    }

    pub const fn image_file_required() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40011, "Image file is required")
    }

    pub const fn image_file_type_not_allowed() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            40012,
            "Image file type is not allowed",
        )
    }

    pub const fn image_file_too_large() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40013, "Image file is too large")
    }

    pub const fn game_field_invalid() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40014, "Game field is invalid")
    }

    pub const fn download_link_invalid() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40015, "Download link is invalid")
    }

    pub const fn resource_file_required() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40016, "Resource file is required")
    }

    pub const fn resource_file_too_large() -> Self {
        Self::new(StatusCode::BAD_REQUEST, 40017, "Resource file is too large")
    }

    pub const fn resource_upload_failed() -> Self {
        Self::new(StatusCode::BAD_GATEWAY, 50201, "Resource upload failed")
    }

    pub const fn invalid_credentials() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, 40101, "Invalid email or password")
    }

    pub const fn authentication_required() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            40102,
            "Authentication is required",
        )
    }

    pub const fn invalid_token() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            40103,
            "Token is invalid or expired",
        )
    }

    pub const fn current_password_incorrect() -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            40104,
            "Current password is incorrect",
        )
    }

    pub const fn permission_denied() -> Self {
        Self::new(StatusCode::FORBIDDEN, 40301, "Permission denied")
    }

    pub const fn user_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40401, "User not found")
    }

    pub const fn uploaded_file_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40402, "Uploaded file not found")
    }

    pub const fn game_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40403, "Game not found")
    }

    pub const fn comment_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40404, "Comment not found")
    }

    pub const fn category_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40405, "Category not found")
    }

    pub const fn tag_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40406, "Tag not found")
    }

    pub const fn download_link_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, 40407, "Download link not found")
    }

    pub const fn username_taken() -> Self {
        Self::new(StatusCode::CONFLICT, 40901, "Username is already taken")
    }

    pub const fn email_registered() -> Self {
        Self::new(StatusCode::CONFLICT, 40902, "Email is already registered")
    }

    pub const fn internal() -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            50000,
            "Internal server error",
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(ApiResponse::error(self.code, Value::Null, self.message)),
        )
            .into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
