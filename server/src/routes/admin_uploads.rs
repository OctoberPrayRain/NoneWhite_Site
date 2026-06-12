use axum::{
    extract::{Multipart, State},
    http::HeaderMap,
    response::Json,
    routing::post,
    Router,
};
use tokio::fs;

use crate::{
    dto::uploads::ImageUploadResponse,
    error::{AppError, AppResult},
    middleware::auth,
    response::ApiResponse,
    services::upload_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/admin/uploads/images", post(upload_image))
}

async fn upload_image(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ImageUploadResponse>>> {
    let admin = auth::admin_user(&headers, &state.config.auth, &state.db_pool).await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::image_file_required())?
    {
        if field.name() != Some("image") {
            continue;
        }

        let content_type = field.content_type().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::image_file_required())?;
        let extension = upload_service::validate_admin_image_file(
            content_type.as_deref(),
            &bytes,
            state.config.upload.max_image_size_bytes,
        )?;
        let image_dir = state.config.upload.upload_dir.join("images");
        fs::create_dir_all(&image_dir)
            .await
            .map_err(|_| AppError::internal())?;

        let file_name = upload_service::stored_file_name("image", admin.id, extension)?;
        let file_path = image_dir.join(&file_name);
        fs::write(&file_path, &bytes)
            .await
            .map_err(|_| AppError::internal())?;

        let image_url = format!(
            "{}/images/{}",
            state.config.upload.public_base_url, file_name
        );

        return Ok(Json(ApiResponse::success(
            ImageUploadResponse { image_url },
            "Image uploaded successfully",
        )));
    }

    Err(AppError::image_file_required())
}
