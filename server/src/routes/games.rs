use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};

use crate::{
    dto::games::{
        CategoryResponse, CreateGameRequest, GameListQuery, GameListResponse, GameResponse,
        TagResponse,
    },
    error::AppResult,
    middleware::auth,
    response::ApiResponse,
    services::game_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/games", get(list_games))
        .route("/api/games/submissions", post(submit_game))
        .route("/api/games/{id}", get(get_game_detail))
        .route("/api/categories", get(list_categories))
        .route("/api/tags", get(list_tags))
}

async fn list_games(
    State(state): State<AppState>,
    Query(query): Query<GameListQuery>,
) -> AppResult<Json<ApiResponse<GameListResponse>>> {
    let response = game_service::list_games(&state.db_pool, query).await?;

    Ok(Json(ApiResponse::success(response, "Games loaded")))
}

async fn submit_game(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreateGameRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<GameResponse>>)> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;
    let response =
        game_service::submit_game(&state.db_pool, &state.config.openlist, user_id, request).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(response, "Game submitted for review")),
    ))
}

async fn get_game_detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<GameResponse>>> {
    let response = game_service::get_game_detail(&state.db_pool, id).await?;

    Ok(Json(ApiResponse::success(response, "Game detail loaded")))
}

async fn list_categories(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<CategoryResponse>>>> {
    let response = game_service::list_categories(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(response, "Categories loaded")))
}

async fn list_tags(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<TagResponse>>>> {
    let response = game_service::list_tags(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(response, "Tags loaded")))
}
