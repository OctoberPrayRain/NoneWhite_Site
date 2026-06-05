mod test;

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::Value;

use crate::response::ApiResponse;

pub fn api_routes() -> Router {
    Router::new().route("/api/test", get(test::get_test_status))
}

pub async fn not_found() -> (StatusCode, Json<ApiResponse<Value>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::error(
            404,
            Value::Null,
            "API endpoint not found",
        )),
    )
}
