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
    validate_user_image_file(content_type, bytes, max_size_bytes)
}

pub(crate) fn validate_user_image_file(
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

pub(crate) fn validate_resource_file(bytes: &[u8], max_size_bytes: usize) -> AppResult<()> {
    if bytes.is_empty() {
        return Err(AppError::resource_file_required());
    }

    if bytes.len() > max_size_bytes {
        return Err(AppError::resource_file_too_large());
    }

    Ok(())
}

pub(crate) fn stored_file_name(prefix: &str, owner_id: i64, extension: &str) -> AppResult<String> {
    let timestamp = current_unix_millis()?;

    Ok(format!("{prefix}-{owner_id}-{timestamp}.{extension}"))
}

pub(crate) fn stored_resource_file_name(
    prefix: &str,
    owner_id: i64,
    original_file_name: Option<&str>,
) -> AppResult<String> {
    let timestamp = current_unix_nanos()?;
    let base = format!("{prefix}-{owner_id}-{timestamp}");

    Ok(match original_file_name.and_then(safe_extension) {
        Some(extension) => format!("{base}.{extension}"),
        None => base,
    })
}

pub(crate) fn is_safe_file_name(file_name: &str) -> bool {
    !file_name.is_empty()
        && !file_name.contains("..")
        && file_name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_'))
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

fn current_unix_millis() -> AppResult<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::internal())?
        .as_millis())
}

fn current_unix_nanos() -> AppResult<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::internal())?
        .as_nanos())
}

fn safe_extension(file_name: &str) -> Option<String> {
    let file_name = file_name.rsplit(['/', '\\']).next()?;
    let (_, extension) = file_name.rsplit_once('.')?;
    let extension = extension.trim().to_ascii_lowercase();

    if extension.is_empty()
        || extension.len() > 16
        || !extension.chars().all(|ch| ch.is_ascii_alphanumeric())
    {
        return None;
    }

    Some(extension)
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

    #[test]
    fn validate_resource_file_accepts_non_empty_bounded_bytes() {
        validate_resource_file(b"archive-bytes", 1024).expect("bounded resource should pass");
    }

    #[test]
    fn validate_resource_file_rejects_empty_resource() {
        let error = validate_resource_file(b"", 1024).expect_err("empty resource should fail");

        assert_eq!(error.code, 40016);
    }

    #[test]
    fn validate_resource_file_rejects_oversized_resource() {
        let error = validate_resource_file(b"archive-bytes", 4)
            .expect_err("oversized resource should fail");

        assert_eq!(error.code, 40017);
    }

    #[test]
    fn stored_resource_file_name_uses_safe_generated_name_and_extension() {
        let file_name = stored_resource_file_name("resource", 9, Some("../Release.ZIP"))
            .expect("resource name should be generated");

        assert!(file_name.starts_with("resource-9-"));
        assert!(file_name.ends_with(".zip"));
        assert!(is_safe_file_name(&file_name));
        assert!(!file_name.contains("Release"));
    }

    #[test]
    fn stored_resource_file_name_drops_unsafe_extension() {
        let file_name = stored_resource_file_name("resource", 9, Some("archive.tar.gz?secret"))
            .expect("resource name should be generated");

        assert!(file_name.starts_with("resource-9-"));
        assert!(!file_name.contains('?'));
        assert!(is_safe_file_name(&file_name));
    }
}
