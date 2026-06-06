use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
    routing::{get, patch},
    Router,
};
use serde_json::{json, Value};

use crate::{
    dto::users::{ChangePasswordRequest, UpdateUserProfileRequest, UserResponse},
    error::AppResult,
    middleware::auth,
    response::ApiResponse,
    services::user_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/users/me", get(get_current_user).patch(update_profile))
        .route("/api/users/me/password", patch(change_password))
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
