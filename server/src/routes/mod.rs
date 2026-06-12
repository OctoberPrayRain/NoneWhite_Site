mod auth;
mod games;
mod interactions;
mod test;
mod uploads;
mod users;

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::Value;

use crate::response::ApiResponse;
use crate::state::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api/test", get(test::get_test_status))
        .merge(auth::routes())
        .merge(games::routes())
        .merge(interactions::routes())
        .merge(users::routes())
        .merge(uploads::routes())
}

pub async fn not_found() -> (StatusCode, Json<ApiResponse<Value>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::error(
            40400,
            Value::Null,
            "API endpoint not found",
        )),
    )
}
