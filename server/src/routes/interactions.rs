use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde_json::{json, Value};

use crate::{
    dto::{
        games::GameListResponse,
        interactions::{
            CommentListQuery, CommentListResponse, CommentResponse, CreateCommentRequest,
            FavoriteStatusResponse, LikeStatusResponse,
        },
    },
    error::AppResult,
    middleware::auth,
    response::ApiResponse,
    services::interaction_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/games/{game_id}/comments",
            get(list_comments).post(create_comment),
        )
        .route("/api/comments/{id}", delete(delete_comment))
        .route(
            "/api/games/{game_id}/like",
            post(like_game).delete(unlike_game),
        )
        .route(
            "/api/games/{game_id}/favorite",
            post(favorite_game).delete(unfavorite_game),
        )
        .route("/api/users/me/favorites", get(list_my_favorites))
}

async fn list_comments(
    State(state): State<AppState>,
    Path(game_id): Path<i64>,
    Query(query): Query<CommentListQuery>,
) -> AppResult<Json<ApiResponse<CommentListResponse>>> {
    let response = interaction_service::list_comments(&state.db_pool, game_id, query).await?;

    Ok(Json(ApiResponse::success(response, "Comments loaded")))
}

async fn create_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
    Json(request): Json<CreateCommentRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<CommentResponse>>)> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response =
        interaction_service::create_comment(&state.db_pool, user_id, game_id, request).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            response,
            "Comment created successfully",
        )),
    ))
}

async fn delete_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    interaction_service::delete_comment(&state.db_pool, user_id, id).await?;

    Ok(Json(ApiResponse::success(
        json!({}),
        "Comment deleted successfully",
    )))
}

async fn like_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<LikeStatusResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response = interaction_service::like_game(&state.db_pool, user_id, game_id).await?;

    Ok(Json(ApiResponse::success(response, "Game liked")))
}

async fn unlike_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<LikeStatusResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response = interaction_service::unlike_game(&state.db_pool, user_id, game_id).await?;

    Ok(Json(ApiResponse::success(response, "Game unliked")))
}

async fn favorite_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<FavoriteStatusResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response = interaction_service::favorite_game(&state.db_pool, user_id, game_id).await?;

    Ok(Json(ApiResponse::success(response, "Game favorited")))
}

async fn unfavorite_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<FavoriteStatusResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response = interaction_service::unfavorite_game(&state.db_pool, user_id, game_id).await?;

    Ok(Json(ApiResponse::success(response, "Game unfavorited")))
}

async fn list_my_favorites(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<CommentListQuery>,
) -> AppResult<Json<ApiResponse<GameListResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response = interaction_service::list_my_favorites(&state.db_pool, user_id, query).await?;

    Ok(Json(ApiResponse::success(response, "Favorites loaded")))
}
