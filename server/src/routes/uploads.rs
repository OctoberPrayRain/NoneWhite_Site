use axum::{
    extract::{Multipart, Path, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use tokio::fs;

use crate::{
    dto::uploads::{ImageUploadResponse, ResourceUploadResponse},
    error::{AppError, AppResult},
    middleware::auth,
    response::ApiResponse,
    services::upload_service,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/uploads/images", post(upload_image))
        .route("/api/uploads/resources", post(upload_resource))
        .route("/uploads/avatars/{file_name}", get(get_avatar))
        .route("/uploads/images/{file_name}", get(get_image))
}

async fn upload_image(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ImageUploadResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;

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
        let extension = upload_service::validate_user_image_file(
            content_type.as_deref(),
            &bytes,
            state.config.upload.max_image_size_bytes,
        )?;
        let image_dir = state.config.upload.upload_dir.join("images");
        fs::create_dir_all(&image_dir)
            .await
            .map_err(|_| AppError::internal())?;

        let file_name = upload_service::stored_file_name("image", user_id, extension)?;
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

async fn upload_resource(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ResourceUploadResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::resource_file_required())?
    {
        if field.name() != Some("resource") {
            continue;
        }

        let original_file_name = field.file_name().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::resource_file_required())?;
        upload_service::validate_resource_file(
            &bytes,
            state.config.upload.max_resource_size_bytes,
        )?;
        let resource_dir = state.config.upload.upload_dir.join("resources");
        fs::create_dir_all(&resource_dir)
            .await
            .map_err(|_| AppError::internal())?;

        let file_name = upload_service::stored_resource_file_name(
            "resource",
            user_id,
            original_file_name.as_deref(),
        )?;
        let file_path = resource_dir.join(&file_name);
        fs::write(&file_path, &bytes)
            .await
            .map_err(|_| AppError::internal())?;

        let resource_url = format!(
            "{}/resources/{}",
            state.config.upload.public_base_url, file_name
        );

        return Ok(Json(ApiResponse::success(
            ResourceUploadResponse {
                resource_url,
                file_name,
                file_size: bytes.len(),
            },
            "Resource uploaded successfully",
        )));
    }

    Err(AppError::resource_file_required())
}

async fn get_avatar(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> AppResult<impl IntoResponse> {
    let content_type =
        avatar_content_type(&file_name).ok_or_else(AppError::uploaded_file_not_found)?;
    if !upload_service::is_safe_file_name(&file_name) {
        return Err(AppError::uploaded_file_not_found());
    }

    let file_path = state
        .config
        .upload
        .upload_dir
        .join("avatars")
        .join(file_name);
    let bytes = fs::read(file_path)
        .await
        .map_err(|_| AppError::uploaded_file_not_found())?;

    Ok((
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        bytes,
    ))
}

async fn get_image(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> AppResult<impl IntoResponse> {
    let content_type =
        image_content_type(&file_name).ok_or_else(AppError::uploaded_file_not_found)?;
    if !upload_service::is_safe_file_name(&file_name) {
        return Err(AppError::uploaded_file_not_found());
    }

    let file_path = state
        .config
        .upload
        .upload_dir
        .join("images")
        .join(file_name);
    let bytes = fs::read(file_path)
        .await
        .map_err(|_| AppError::uploaded_file_not_found())?;

    Ok((
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        bytes,
    ))
}

fn avatar_content_type(file_name: &str) -> Option<&'static str> {
    image_content_type(file_name)
}

fn image_content_type(file_name: &str) -> Option<&'static str> {
    if file_name.ends_with(".png") {
        Some("image/png")
    } else if file_name.ends_with(".jpg") {
        Some("image/jpeg")
    } else if file_name.ends_with(".webp") {
        Some("image/webp")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_file_name_rejects_traversal() {
        assert!(!upload_service::is_safe_file_name("../avatar.png"));
        assert!(!upload_service::is_safe_file_name("..avatar.png"));
    }

    #[test]
    fn avatar_content_type_accepts_whitelisted_extensions() {
        assert_eq!(avatar_content_type("a.png"), Some("image/png"));
        assert_eq!(avatar_content_type("a.jpg"), Some("image/jpeg"));
        assert_eq!(avatar_content_type("a.webp"), Some("image/webp"));
        assert_eq!(avatar_content_type("a.gif"), None);
    }

    #[test]
    fn image_content_type_reuses_image_whitelist() {
        assert_eq!(image_content_type("cover.png"), Some("image/png"));
        assert_eq!(image_content_type("cover.gif"), None);
    }
}
