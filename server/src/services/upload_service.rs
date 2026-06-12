use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AppError, AppResult};

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
const JPEG_SIGNATURE: &[u8] = b"\xff\xd8\xff";

pub(crate) fn validate_avatar_file(
    content_type: Option<&str>,
    bytes: &[u8],
    max_size_bytes: usize,
) -> AppResult<&'static str> {
    validate_image_file(
        content_type,
        bytes,
        max_size_bytes,
        AppError::avatar_file_required(),
        AppError::avatar_file_type_not_allowed(),
        AppError::avatar_file_too_large(),
    )
}

pub(crate) fn validate_admin_image_file(
    content_type: Option<&str>,
    bytes: &[u8],
    max_size_bytes: usize,
) -> AppResult<&'static str> {
    validate_image_file(
        content_type,
        bytes,
        max_size_bytes,
        AppError::image_file_required(),
        AppError::image_file_type_not_allowed(),
        AppError::image_file_too_large(),
    )
}

pub(crate) fn stored_file_name(prefix: &str, owner_id: i64, extension: &str) -> AppResult<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::internal())?
        .as_millis();

    Ok(format!("{prefix}-{owner_id}-{timestamp}.{extension}"))
}

fn validate_image_file(
    content_type: Option<&str>,
    bytes: &[u8],
    max_size_bytes: usize,
    required_error: AppError,
    type_error: AppError,
    size_error: AppError,
) -> AppResult<&'static str> {
    if bytes.is_empty() {
        return Err(required_error);
    }

    if bytes.len() > max_size_bytes {
        return Err(size_error);
    }

    match content_type {
        Some("image/png") if bytes.starts_with(PNG_SIGNATURE) => Ok("png"),
        Some("image/jpeg") if bytes.starts_with(JPEG_SIGNATURE) => Ok("jpg"),
        Some("image/webp") if is_webp(bytes) => Ok("webp"),
        _ => Err(type_error),
    }
}

fn is_webp(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_avatar_file_accepts_png_signature() {
        let mut bytes = PNG_SIGNATURE.to_vec();
        bytes.extend_from_slice(b"avatar-bytes");

        let extension =
            validate_avatar_file(Some("image/png"), &bytes, 1024).expect("valid png should pass");

        assert_eq!(extension, "png");
    }

    #[test]
    fn validate_avatar_file_rejects_oversized_file_with_avatar_code() {
        let error = validate_avatar_file(Some("image/png"), PNG_SIGNATURE, 4)
            .expect_err("oversized avatar should fail");

        assert_eq!(error.code, 40008);
    }

    #[test]
    fn validate_admin_image_file_rejects_unknown_content_type_with_image_code() {
        let error = validate_admin_image_file(Some("text/plain"), b"hello", 1024)
            .expect_err("text file should fail");

        assert_eq!(error.code, 40012);
    }
}
