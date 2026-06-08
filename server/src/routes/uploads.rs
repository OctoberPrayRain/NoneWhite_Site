use axum::{
    extract::{Path, State},
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::fs;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/uploads/avatars/{file_name}", get(get_avatar))
}

async fn get_avatar(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> AppResult<impl IntoResponse> {
    let content_type =
        avatar_content_type(&file_name).ok_or_else(AppError::uploaded_file_not_found)?;
    if !is_safe_file_name(&file_name) {
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

fn avatar_content_type(file_name: &str) -> Option<&'static str> {
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

fn is_safe_file_name(file_name: &str) -> bool {
    !file_name.is_empty()
        && !file_name.contains("..")
        && file_name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_file_name_rejects_traversal() {
        assert!(!is_safe_file_name("../avatar.png"));
        assert!(!is_safe_file_name("..avatar.png"));
    }

    #[test]
    fn avatar_content_type_accepts_whitelisted_extensions() {
        assert_eq!(avatar_content_type("a.png"), Some("image/png"));
        assert_eq!(avatar_content_type("a.jpg"), Some("image/jpeg"));
        assert_eq!(avatar_content_type("a.webp"), Some("image/webp"));
        assert_eq!(avatar_content_type("a.gif"), None);
    }
}
