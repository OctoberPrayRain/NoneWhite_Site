use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};

use crate::{
    dto::auth::{AuthTokenResponse, LoginRequest, RegisterRequest},
    dto::users::UserResponse,
    error::AppResult,
    response::ApiResponse,
    services::auth_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UserResponse>>)> {
    let user = auth_service::register_user(&state.db_pool, request).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(user, "User registered successfully")),
    ))
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<AuthTokenResponse>>> {
    let token_response =
        auth_service::login_user(&state.db_pool, &state.config.auth, request).await?;

    Ok(Json(ApiResponse::success(
        token_response,
        "Login successful",
    )))
}
