use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post, put},
    Router,
};
use serde_json::{json, Value};

use crate::{
    dto::{
        download_links::{DownloadLinkRequest, DownloadLinkResponse},
        games::{
            CreateGameRequest, GameListQuery, GameListResponse, GameResponse, UpdateGameRequest,
        },
    },
    error::{AppError, AppResult},
    middleware::auth,
    response::ApiResponse,
    services::{
        download_link_service::{self, PublicDownloadTarget},
        game_service,
    },
    state::AppState,
};
use tokio::fs;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/admin/games", get(list_games).post(create_game))
        .route("/api/admin/games/pending", get(list_pending_games))
        .route("/api/admin/games/{game_id}/approve", post(approve_game))
        .route(
            "/api/admin/games/{game_id}",
            put(update_game).delete(delete_game),
        )
        .route(
            "/api/admin/games/{game_id}/download-links",
            get(list_admin_download_links).post(create_download_link),
        )
        .route(
            "/api/admin/games/{game_id}/download-links/{id}",
            put(update_download_link).delete(delete_download_link),
        )
        .route(
            "/api/games/{game_id}/download-links",
            get(list_public_download_links),
        )
        .route(
            "/api/games/{game_id}/download-links/{id}/download",
            get(download_public_download_link),
        )
}

async fn list_games(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<GameListQuery>,
) -> AppResult<Json<ApiResponse<GameListResponse>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = game_service::list_admin_games(&state.db_pool, query).await?;

    Ok(Json(ApiResponse::success(response, "Admin games loaded")))
}

async fn list_pending_games(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<GameListQuery>,
) -> AppResult<Json<ApiResponse<GameListResponse>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = game_service::list_pending_games(&state.db_pool, query).await?;

    Ok(Json(ApiResponse::success(response, "Pending games loaded")))
}

async fn create_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateGameRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<GameResponse>>)> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = game_service::create_game(&state.db_pool, request).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "Game created successfully")),
    ))
}

async fn update_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
    Json(request): Json<UpdateGameRequest>,
) -> AppResult<Json<ApiResponse<GameResponse>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = game_service::update_game(&state.db_pool, game_id, request).await?;

    Ok(Json(ApiResponse::success(
        response,
        "Game updated successfully",
    )))
}

async fn approve_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<GameResponse>>> {
    let admin = auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = game_service::approve_game(&state.db_pool, game_id, admin.id).await?;

    Ok(Json(ApiResponse::success(
        response,
        "Game approved successfully",
    )))
}

async fn delete_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    game_service::delete_game(&state.db_pool, game_id).await?;

    Ok(Json(ApiResponse::success(
        json!({}),
        "Game deleted successfully",
    )))
}

async fn list_admin_download_links(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<Vec<DownloadLinkResponse>>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response = download_link_service::list_download_links(&state.db_pool, game_id).await?;

    Ok(Json(ApiResponse::success(
        response,
        "Download links loaded",
    )))
}

async fn list_public_download_links(
    State(state): State<AppState>,
    Path(game_id): Path<i64>,
) -> AppResult<Json<ApiResponse<Vec<DownloadLinkResponse>>>> {
    let response =
        download_link_service::list_public_download_links(&state.db_pool, game_id).await?;

    Ok(Json(ApiResponse::success(
        response,
        "Download links loaded",
    )))
}

async fn download_public_download_link(
    State(state): State<AppState>,
    Path((game_id, id)): Path<(i64, i64)>,
) -> AppResult<Response> {
    let target = download_link_service::public_download_target(
        &state.db_pool,
        &state.config.openlist,
        game_id,
        id,
    )
    .await?;

    match target {
        PublicDownloadTarget::OpenList(target) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("x-accel-redirect"),
                HeaderValue::from_str(&target.x_accel_redirect)
                    .map_err(|_| AppError::internal())?,
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&target.content_disposition)
                    .map_err(|_| AppError::internal())?,
            );

            Ok((StatusCode::OK, headers, ()).into_response())
        }
        PublicDownloadTarget::LocalResource(target) => {
            let file_path = state
                .config
                .upload
                .upload_dir
                .join("resources")
                .join(&target.file_name);
            let bytes = fs::read(file_path)
                .await
                .map_err(|_| AppError::uploaded_file_not_found())?;
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
            headers.insert(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_str(&target.content_disposition)
                    .map_err(|_| AppError::internal())?,
            );

            Ok((StatusCode::OK, headers, bytes).into_response())
        }
    }
}

async fn create_download_link(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<i64>,
    Json(request): Json<DownloadLinkRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<DownloadLinkResponse>>)> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response =
        download_link_service::create_download_link(&state.db_pool, game_id, request).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            response,
            "Download link created successfully",
        )),
    ))
}

async fn update_download_link(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((game_id, id)): Path<(i64, i64)>,
    Json(request): Json<DownloadLinkRequest>,
) -> AppResult<Json<ApiResponse<DownloadLinkResponse>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    let response =
        download_link_service::update_download_link(&state.db_pool, game_id, id, request).await?;

    Ok(Json(ApiResponse::success(
        response,
        "Download link updated successfully",
    )))
}

async fn delete_download_link(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((game_id, id)): Path<(i64, i64)>,
) -> AppResult<Json<ApiResponse<Value>>> {
    auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;
    download_link_service::delete_download_link(&state.db_pool, game_id, id).await?;

    Ok(Json(ApiResponse::success(
        json!({}),
        "Download link deleted successfully",
    )))
}
