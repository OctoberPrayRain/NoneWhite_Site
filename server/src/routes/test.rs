use axum::response::Json;
use serde_json::{json, Value};

use crate::response::ApiResponse;

pub async fn get_test_status() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::success(
        json!({
            "service": "NoneWhite_Site Rust API",
            "status": "ok"
        }),
        "Backend test API is running",
    ))
}
