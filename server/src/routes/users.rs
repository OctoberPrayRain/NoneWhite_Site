use axum::{
    extract::{Multipart, State},
    http::HeaderMap,
    response::Json,
    routing::{get, patch, post},
    Router,
};
use serde_json::{json, Value};
use tokio::fs;

use crate::{
    dto::users::{
        AvatarUploadResponse, ChangePasswordRequest, UpdateUserProfileRequest, UserResponse,
    },
    error::{AppError, AppResult},
    middleware::auth,
    response::ApiResponse,
    services::{upload_service, user_service},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/users/me", get(get_current_user).patch(update_profile))
        .route("/api/users/me/password", patch(change_password))
        .route("/api/users/me/avatar", post(upload_avatar))
}

async fn get_current_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let user = user_service::get_current_user(&state.db_pool, user_id).await?;

    Ok(Json(ApiResponse::success(user, "Current user loaded")))
}

async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<UpdateUserProfileRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let user = user_service::update_profile(&state.db_pool, user_id, request).await?;

    Ok(Json(ApiResponse::success(
        user,
        "Profile updated successfully",
    )))
}

async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ChangePasswordRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    user_service::change_password(&state.db_pool, user_id, request).await?;

    Ok(Json(ApiResponse::success(
        json!({}),
        "Password changed successfully",
    )))
}

async fn upload_avatar(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<AvatarUploadResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::avatar_file_required())?
    {
        if field.name() != Some("avatar") {
            continue;
        }

        let content_type = field.content_type().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::avatar_file_required())?;
        let extension = upload_service::validate_avatar_file(
            content_type.as_deref(),
            &bytes,
            state.config.upload.max_avatar_size_bytes,
        )?;
        let avatar_dir = state.config.upload.upload_dir.join("avatars");
        fs::create_dir_all(&avatar_dir)
            .await
            .map_err(|_| AppError::internal())?;

        let file_name = upload_service::stored_file_name("user", user_id, extension)?;
        let file_path = avatar_dir.join(&file_name);
        fs::write(&file_path, &bytes)
            .await
            .map_err(|_| AppError::internal())?;

        let avatar_url = format!(
            "{}/avatars/{}",
            state.config.upload.public_base_url, file_name
        );
        let response = match user_service::update_avatar(&state.db_pool, user_id, &avatar_url).await
        {
            Ok(response) => response,
            Err(error) => {
                fs::remove_file(&file_path).await.ok();
                return Err(error);
            }
        };

        return Ok(Json(ApiResponse::success(
            response,
            "Avatar uploaded successfully",
        )));
    }

    Err(AppError::avatar_file_required())
}
