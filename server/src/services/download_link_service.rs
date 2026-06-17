use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::PgPool;

use crate::{
    config::OpenListConfig,
    dto::download_links::{DownloadLinkRequest, DownloadLinkResponse},
    error::{AppError, AppResult},
    repositories::{download_link_repository, game_repository},
    services::upload_service,
};

type HmacSha256 = Hmac<Sha256>;

const OPENLIST_SCHEME: &str = "openlist:";
const OPENLIST_GOOGLE_DRIVE_PREFIX: &str = "openlist:/GoogleDrive/";
const OPENLIST_X_ACCEL_PREFIX: &str = "/internal/openlist";
const LOCAL_RESOURCE_PREFIX: &str = "/uploads/resources/";
const PUBLIC_OPENLIST_PLATFORM: &str = "本站托管资源";

#[derive(Clone, Debug)]
pub struct OpenListDownloadTarget {
    pub x_accel_redirect: String,
    pub content_disposition: String,
}

#[derive(Clone, Debug)]
pub struct LocalResourceDownloadTarget {
    pub file_name: String,
    pub content_disposition: String,
}

#[derive(Clone, Debug)]
pub enum PublicDownloadTarget {
    OpenList(OpenListDownloadTarget),
    LocalResource(LocalResourceDownloadTarget),
}

pub async fn list_download_links(
    pool: &PgPool,
    game_id: i64,
) -> AppResult<Vec<DownloadLinkResponse>> {
    ensure_game_exists(pool, game_id).await?;

    list_download_links_for_game(pool, game_id).await
}

pub async fn list_public_download_links(
    pool: &PgPool,
    game_id: i64,
) -> AppResult<Vec<DownloadLinkResponse>> {
    ensure_approved_game_exists(pool, game_id).await?;

    let links = list_download_links_for_game(pool, game_id)
        .await?
        .into_iter()
        .map(to_public_download_link_response)
        .collect();

    Ok(links)
}

async fn list_download_links_for_game(
    pool: &PgPool,
    game_id: i64,
) -> AppResult<Vec<DownloadLinkResponse>> {
    let links = download_link_repository::list_download_links(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?
        .into_iter()
        .map(DownloadLinkResponse::from)
        .collect();

    Ok(links)
}

pub async fn create_download_link(
    pool: &PgPool,
    game_id: i64,
    request: DownloadLinkRequest,
) -> AppResult<DownloadLinkResponse> {
    ensure_game_exists(pool, game_id).await?;
    let request = validate_download_link_request(request)?;
    let link = download_link_repository::create_download_link(pool, game_id, &request)
        .await
        .map_err(map_download_link_write_error)?;

    Ok(DownloadLinkResponse::from(link))
}

pub async fn update_download_link(
    pool: &PgPool,
    game_id: i64,
    id: i64,
    request: DownloadLinkRequest,
) -> AppResult<DownloadLinkResponse> {
    ensure_game_exists(pool, game_id).await?;
    if id <= 0 {
        return Err(AppError::download_link_not_found());
    }

    let request = validate_download_link_request(request)?;
    let link = download_link_repository::update_download_link(pool, game_id, id, &request)
        .await
        .map_err(map_download_link_write_error)?
        .ok_or_else(AppError::download_link_not_found)?;

    Ok(DownloadLinkResponse::from(link))
}

pub async fn delete_download_link(pool: &PgPool, game_id: i64, id: i64) -> AppResult<()> {
    ensure_game_exists(pool, game_id).await?;
    if id <= 0 {
        return Err(AppError::download_link_not_found());
    }

    let rows_affected = download_link_repository::delete_download_link(pool, game_id, id)
        .await
        .map_err(|_| AppError::internal())?;
    if rows_affected == 0 {
        return Err(AppError::download_link_not_found());
    }

    Ok(())
}

pub async fn openlist_download_accel_target(
    pool: &PgPool,
    openlist_config: &OpenListConfig,
    game_id: i64,
    id: i64,
) -> AppResult<OpenListDownloadTarget> {
    match public_download_target(pool, openlist_config, game_id, id).await? {
        PublicDownloadTarget::OpenList(target) => Ok(target),
        PublicDownloadTarget::LocalResource(_) => Err(AppError::download_link_not_found()),
    }
}

pub async fn public_download_target(
    pool: &PgPool,
    openlist_config: &OpenListConfig,
    game_id: i64,
    id: i64,
) -> AppResult<PublicDownloadTarget> {
    ensure_approved_game_exists(pool, game_id).await?;
    if id <= 0 {
        return Err(AppError::download_link_not_found());
    }

    let link = download_link_repository::find_download_link(pool, game_id, id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::download_link_not_found)?;

    if is_openlist_url(&link.url) {
        let marker = validate_openlist_marker(link.url)?;
        let file_name = openlist_download_file_name(&marker)?;

        return Ok(PublicDownloadTarget::OpenList(OpenListDownloadTarget {
            x_accel_redirect: to_x_accel_redirect_target(&marker, &openlist_config.token)?,
            content_disposition: content_disposition_attachment(&file_name)?,
        }));
    }

    if is_local_resource_url(&link.url) {
        let marker = validate_local_resource_marker(link.url)?;
        let file_name = local_resource_file_name(&marker)?;

        return Ok(PublicDownloadTarget::LocalResource(
            LocalResourceDownloadTarget {
                content_disposition: content_disposition_attachment(&file_name)?,
                file_name,
            },
        ));
    }

    Err(AppError::download_link_not_found())
}

pub(crate) fn validate_download_link_request(
    mut request: DownloadLinkRequest,
) -> AppResult<DownloadLinkRequest> {
    request.platform = required_text(request.platform)?;
    request.url = validate_download_url(request.url)?;
    request.extract_code = optional_text(request.extract_code);
    request.password = optional_text(request.password);
    request.file_size = optional_text(request.file_size);

    Ok(request)
}

pub(crate) fn validate_submission_download_link_request(
    mut request: DownloadLinkRequest,
    openlist_config: &OpenListConfig,
    user_id: i64,
) -> AppResult<DownloadLinkRequest> {
    request.platform = required_text(request.platform)?;
    request.url = validate_submission_download_url(request.url, openlist_config, user_id)?;
    request.extract_code = optional_text(request.extract_code);
    request.password = optional_text(request.password);
    request.file_size = optional_text(request.file_size);

    Ok(request)
}

fn required_text(value: String) -> AppResult<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(AppError::download_link_invalid());
    }

    Ok(value.to_string())
}

fn validate_download_url(value: String) -> AppResult<String> {
    let value = required_text(value)?;
    let normalized = value.to_ascii_lowercase();

    if is_openlist_url(&value) {
        return validate_openlist_marker(value);
    }

    if is_local_resource_url(&value) {
        return validate_local_resource_marker(value);
    }

    if value.chars().any(char::is_control)
        || !(normalized.starts_with("https://") || normalized.starts_with("http://"))
    {
        return Err(AppError::download_link_invalid());
    }

    Ok(value)
}

fn validate_submission_download_url(
    value: String,
    openlist_config: &OpenListConfig,
    user_id: i64,
) -> AppResult<String> {
    let value = required_text(value)?;
    let normalized = value.to_ascii_lowercase();

    if is_openlist_url(&value) {
        return validate_uploaded_openlist_resource_marker(value, openlist_config, user_id);
    }

    if is_local_resource_url(&value) {
        return validate_local_resource_marker(value);
    }

    if value.chars().any(char::is_control)
        || !(normalized.starts_with("https://") || normalized.starts_with("http://"))
    {
        return Err(AppError::download_link_invalid());
    }

    Ok(value)
}

fn is_openlist_url(value: &str) -> bool {
    value.trim().starts_with(OPENLIST_SCHEME)
}

fn is_local_resource_url(value: &str) -> bool {
    value.trim().starts_with(LOCAL_RESOURCE_PREFIX)
}

fn validate_openlist_marker(value: String) -> AppResult<String> {
    let value = required_text(value)?;
    if value.chars().any(char::is_control)
        || value.contains('?')
        || value.contains('#')
        || value.contains('\\')
        || value.contains('%')
        || !value.starts_with(OPENLIST_GOOGLE_DRIVE_PREFIX)
    {
        return Err(AppError::download_link_invalid());
    }

    let resource_path = &value[OPENLIST_GOOGLE_DRIVE_PREFIX.len()..];
    if resource_path.is_empty()
        || resource_path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(AppError::download_link_invalid());
    }

    Ok(value)
}

fn validate_uploaded_openlist_resource_marker(
    value: String,
    openlist_config: &OpenListConfig,
    user_id: i64,
) -> AppResult<String> {
    if user_id <= 0 {
        return Err(AppError::download_link_invalid());
    }

    let value = validate_openlist_marker(value)?;
    let upload_dir = validate_openlist_marker(openlist_config.resource_upload_dir.clone())?;
    let prefix = format!("{upload_dir}/");
    let file_name = value
        .strip_prefix(&prefix)
        .filter(|file_name| !file_name.contains('/'))
        .ok_or_else(AppError::download_link_invalid)?;

    if is_generated_resource_file_name_for_user(file_name, user_id) {
        Ok(value)
    } else {
        Err(AppError::download_link_invalid())
    }
}

fn is_generated_resource_file_name_for_user(file_name: &str, user_id: i64) -> bool {
    if !upload_service::is_safe_file_name(file_name) {
        return false;
    }

    let prefix = format!("resource-{user_id}-");
    let Some(suffix) = file_name.strip_prefix(&prefix) else {
        return false;
    };
    let (timestamp, extension) = suffix
        .rsplit_once('.')
        .map_or((suffix, None), |(timestamp, extension)| {
            (timestamp, Some(extension))
        });

    if timestamp.is_empty() || !timestamp.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }

    match extension {
        Some(extension) => {
            !extension.is_empty()
                && extension.len() <= 16
                && extension.chars().all(|ch| ch.is_ascii_alphanumeric())
        }
        None => true,
    }
}

fn validate_local_resource_marker(value: String) -> AppResult<String> {
    let value = required_text(value)?;
    if value.chars().any(char::is_control)
        || value.contains('?')
        || value.contains('#')
        || value.contains('\\')
        || value.contains('%')
        || !value.starts_with(LOCAL_RESOURCE_PREFIX)
    {
        return Err(AppError::download_link_invalid());
    }

    let file_name = &value[LOCAL_RESOURCE_PREFIX.len()..];
    if file_name.contains('/') || !upload_service::is_safe_file_name(file_name) {
        return Err(AppError::download_link_invalid());
    }

    Ok(value)
}

fn to_public_download_url(game_id: i64, id: i64, url: &str) -> String {
    if is_protected_download_url(url) {
        format!("/api/games/{game_id}/download-links/{id}/download")
    } else {
        url.to_string()
    }
}

fn to_public_download_link_response(mut link: DownloadLinkResponse) -> DownloadLinkResponse {
    let is_protected = is_protected_download_url(&link.url);
    link.url = to_public_download_url(link.game_id, link.id, &link.url);
    link.platform = if is_protected {
        PUBLIC_OPENLIST_PLATFORM.to_string()
    } else {
        neutralize_public_provider_text(&link.platform)
    };
    link
}

fn is_protected_download_url(value: &str) -> bool {
    is_openlist_url(value) || is_local_resource_url(value)
}

fn neutralize_public_provider_text(value: &str) -> String {
    value
        .replace("OpenList Archive", PUBLIC_OPENLIST_PLATFORM)
        .replace("OpenList", "本站")
        .replace("OPENLIST", "本站")
        .replace("openlist", "本站")
}

fn openlist_download_file_name(marker: &str) -> AppResult<String> {
    let openlist_path = marker
        .strip_prefix(OPENLIST_SCHEME)
        .ok_or_else(AppError::download_link_invalid)?;
    let file_name = openlist_path
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(AppError::download_link_invalid)?;

    Ok(file_name.to_string())
}

fn local_resource_file_name(marker: &str) -> AppResult<String> {
    marker
        .strip_prefix(LOCAL_RESOURCE_PREFIX)
        .filter(|file_name| upload_service::is_safe_file_name(file_name))
        .map(str::to_string)
        .ok_or_else(AppError::download_link_invalid)
}

fn content_disposition_attachment(file_name: &str) -> AppResult<String> {
    if file_name.is_empty() || file_name.chars().any(char::is_control) {
        return Err(AppError::download_link_invalid());
    }

    Ok(format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        ascii_filename_fallback(file_name),
        encode_header_parameter(file_name)
    ))
}

fn ascii_filename_fallback(file_name: &str) -> String {
    let fallback: String = file_name
        .chars()
        .filter_map(|ch| match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | '-' | '_' => Some(ch),
            ' ' => Some('_'),
            _ if ch.is_ascii() => Some('_'),
            _ => None,
        })
        .collect();

    let fallback = fallback.trim_matches('_');
    if fallback.is_empty() || fallback.starts_with('.') {
        format!("download{}", archive_extension(file_name))
    } else {
        fallback.to_string()
    }
}

fn archive_extension(file_name: &str) -> String {
    if let Some(prefix) = file_name.strip_suffix(".tar.zst") {
        if !prefix.is_empty() {
            return ".tar.zst".to_string();
        }
    }

    file_name
        .rsplit_once('.')
        .map(|(_, extension)| format!(".{extension}"))
        .unwrap_or_default()
}

fn encode_header_parameter(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'!'
            | b'#'
            | b'$'
            | b'&'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~' => encoded.push(char::from(byte)),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn to_x_accel_redirect_target(marker: &str, openlist_token: &str) -> AppResult<String> {
    let openlist_path = marker
        .strip_prefix(OPENLIST_SCHEME)
        .expect("validated openlist marker should include openlist scheme");
    let sign = sign_openlist_path(openlist_path, openlist_token)?;

    Ok(format!(
        "{OPENLIST_X_ACCEL_PREFIX}/p{}?sign={sign}",
        encode_uri_path(openlist_path)
    ))
}

fn sign_openlist_path(path: &str, openlist_token: &str) -> AppResult<String> {
    if openlist_token.trim().is_empty() {
        return Err(AppError::internal());
    }

    let mut mac =
        HmacSha256::new_from_slice(openlist_token.as_bytes()).map_err(|_| AppError::internal())?;
    mac.update(format!("{path}:0").as_bytes());
    let hmac = mac.finalize().into_bytes();

    Ok(format!("{}:0", URL_SAFE.encode(hmac)))
}

fn encode_uri_path(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(byte))
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_string();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    })
}

async fn ensure_game_exists(pool: &PgPool, game_id: i64) -> AppResult<()> {
    if game_id <= 0 {
        return Err(AppError::game_not_found());
    }

    game_repository::find_game_by_id(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::game_not_found)?;

    Ok(())
}

async fn ensure_approved_game_exists(pool: &PgPool, game_id: i64) -> AppResult<()> {
    if game_id <= 0 {
        return Err(AppError::game_not_found());
    }

    game_repository::find_approved_game_by_id(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::game_not_found)?;

    Ok(())
}

fn map_download_link_write_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23503") =>
        {
            AppError::game_not_found()
        }
        _ => AppError::internal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_download_link_request_trims_optional_fields() {
        let request = validate_download_link_request(DownloadLinkRequest {
            platform: "  Baidu Netdisk  ".to_string(),
            url: "  https://example.invalid/share  ".to_string(),
            extract_code: Some(" abcd ".to_string()),
            password: Some("  ".to_string()),
            file_size: Some(" 1.2 GiB ".to_string()),
        })
        .expect("valid download link should pass");

        assert_eq!(request.platform, "Baidu Netdisk");
        assert_eq!(request.extract_code.as_deref(), Some("abcd"));
        assert_eq!(request.password, None);
        assert_eq!(request.file_size.as_deref(), Some("1.2 GiB"));
    }

    #[test]
    fn validate_download_link_request_rejects_blank_url() {
        let error = validate_download_link_request(DownloadLinkRequest {
            platform: "Baidu".to_string(),
            url: "  ".to_string(),
            extract_code: None,
            password: None,
            file_size: None,
        })
        .expect_err("blank url should fail");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_download_link_request_rejects_unsafe_url_scheme() {
        let error = validate_download_link_request(DownloadLinkRequest {
            platform: "Baidu".to_string(),
            url: "javascript:alert(1)".to_string(),
            extract_code: None,
            password: None,
            file_size: None,
        })
        .expect_err("unsafe url scheme should fail");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_submission_download_link_request_accepts_uploaded_openlist_marker_for_user() {
        let request = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Uploaded resource".to_string(),
                url: "openlist:/GoogleDrive/NoneWhite/resources/resource-42-1234567890.zip"
                    .to_string(),
                extract_code: None,
                password: None,
                file_size: Some("10 MiB".to_string()),
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect("generated marker under configured dir should pass for owner");

        assert_eq!(
            request.url,
            "openlist:/GoogleDrive/NoneWhite/resources/resource-42-1234567890.zip"
        );
        assert_eq!(request.file_size.as_deref(), Some("10 MiB"));
    }

    #[test]
    fn validate_submission_download_link_request_rejects_arbitrary_openlist_marker() {
        let error = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Uploaded resource".to_string(),
                url: "openlist:/GoogleDrive/private/secret.zip".to_string(),
                extract_code: None,
                password: None,
                file_size: None,
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect_err("public submission should not persist arbitrary OpenList paths");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_submission_download_link_request_rejects_openlist_marker_for_other_user() {
        let error = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Uploaded resource".to_string(),
                url: "openlist:/GoogleDrive/NoneWhite/resources/resource-7-1234567890.zip"
                    .to_string(),
                extract_code: None,
                password: None,
                file_size: None,
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect_err("public submission should only accept own generated upload markers");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_submission_download_link_request_rejects_openlist_marker_outside_configured_dir() {
        let error = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Uploaded resource".to_string(),
                url: "openlist:/GoogleDrive/Other/resources/resource-42-1234567890.zip".to_string(),
                extract_code: None,
                password: None,
                file_size: None,
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect_err("public submission should only accept configured upload dir markers");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_submission_download_link_request_accepts_external_http_link() {
        let request = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Mirror".to_string(),
                url: " https://example.invalid/releases/file.zip ".to_string(),
                extract_code: None,
                password: None,
                file_size: None,
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect("manual external links should remain valid for public submissions");

        assert_eq!(request.url, "https://example.invalid/releases/file.zip");
    }

    #[test]
    fn validate_submission_download_link_request_accepts_legacy_local_resource_marker() {
        let request = validate_submission_download_link_request(
            DownloadLinkRequest {
                platform: "Uploaded resource".to_string(),
                url: "/uploads/resources/resource-42-1234567890.zip".to_string(),
                extract_code: None,
                password: None,
                file_size: None,
            },
            &openlist_config("openlist:/GoogleDrive/NoneWhite/resources"),
            42,
        )
        .expect("legacy local resource markers should remain valid");

        assert_eq!(request.url, "/uploads/resources/resource-42-1234567890.zip");
    }

    #[test]
    fn validate_download_url_accepts_openlist_google_drive_marker() {
        let url = "openlist:/GoogleDrive/abc123/file.zip".to_string();
        validate_download_url(url).expect("openlist:/GoogleDrive/... should be valid");
    }

    #[test]
    fn validate_download_url_accepts_openlist_marker_without_trailing_component() {
        let url = "openlist:/GoogleDrive/abc123".to_string();
        validate_download_url(url).expect("openlist:/GoogleDrive/<id> should be valid");
    }

    #[test]
    fn validate_download_url_accepts_local_resource_marker() {
        let url = "/uploads/resources/resource-9-1234567890.zip".to_string();
        validate_download_url(url).expect("local resource marker should be valid");
    }

    #[test]
    fn validate_openlist_marker_accepts_chinese_and_space_filename() {
        let url = "openlist:/GoogleDrive/ensemble/少女 文件.rar".to_string();
        validate_openlist_marker(url).expect("Chinese and space filenames should be valid");
    }

    #[test]
    fn is_openlist_url_recognizes_openlist_prefix() {
        assert!(is_openlist_url("openlist:/GoogleDrive/abc123/file.zip"));
        assert!(is_openlist_url("openlist:/GoogleDrive/"));
    }

    #[test]
    fn is_openlist_url_rejects_http_and_https() {
        assert!(!is_openlist_url("https://example.com/file.zip"));
        assert!(!is_openlist_url("http://example.com/file.zip"));
    }

    #[test]
    fn is_openlist_url_rejects_empty_and_blank() {
        assert!(!is_openlist_url(""));
        assert!(!is_openlist_url("   "));
    }

    #[test]
    fn is_local_resource_url_recognizes_local_prefix() {
        assert!(is_local_resource_url(
            "/uploads/resources/resource-9-1234567890.zip"
        ));
        assert!(!is_local_resource_url("/uploads/images/cover.png"));
        assert!(!is_local_resource_url("https://example.invalid/file.zip"));
    }

    #[test]
    fn validate_openlist_marker_rejects_disallowed_prefix() {
        let error = validate_openlist_marker("openlist:/MaliciousPrefix/abc".to_string())
            .expect_err("openlist:/MaliciousPrefix/... should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_path_traversal() {
        let error =
            validate_openlist_marker("openlist:/GoogleDrive/../../../etc/passwd".to_string())
                .expect_err("path traversal should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_percent_encoded_path_confusion() {
        let error = validate_openlist_marker("openlist:/GoogleDrive/%2e%2e/secret.rar".to_string())
            .expect_err("percent-encoded traversal should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_backslash_path_separators() {
        let error =
            validate_openlist_marker("openlist:/GoogleDrive/foo\\..\\secret.rar".to_string())
                .expect_err("backslash path separators should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_query_string() {
        let error =
            validate_openlist_marker("openlist:/GoogleDrive/abc123?token=secret".to_string())
                .expect_err("query string should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_fragment() {
        let error = validate_openlist_marker("openlist:/GoogleDrive/abc123#section".to_string())
            .expect_err("fragment should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_openlist_marker_rejects_control_characters() {
        let error = validate_openlist_marker("openlist:/GoogleDrive/abc\n123".to_string())
            .expect_err("control characters should be rejected");
        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_local_resource_marker_rejects_path_traversal() {
        let error =
            validate_local_resource_marker("/uploads/resources/../../secret.zip".to_string())
                .expect_err("local resource traversal should be rejected");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_local_resource_marker_rejects_nested_path() {
        let error =
            validate_local_resource_marker("/uploads/resources/nested/file.zip".to_string())
                .expect_err("local resource marker should name one safe file");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn validate_local_resource_marker_rejects_encoded_path_confusion() {
        let error =
            validate_local_resource_marker("/uploads/resources/%2e%2e-secret.zip".to_string())
                .expect_err("encoded local resource path should be rejected");

        assert_eq!(error.code, 40015);
    }

    #[test]
    fn to_public_download_url_maps_openlist_to_site_local() {
        let result = to_public_download_url(42, 7, "openlist:/GoogleDrive/abc123/file.zip");
        assert_eq!(result, "/api/games/42/download-links/7/download");
    }

    #[test]
    fn to_public_download_url_maps_local_resource_to_site_local() {
        let result = to_public_download_url(42, 7, "/uploads/resources/resource-9-123.zip");
        assert_eq!(result, "/api/games/42/download-links/7/download");
    }

    #[test]
    fn to_public_download_url_preserves_http_urls() {
        let result = to_public_download_url(42, 7, "https://example.com/file.zip");
        assert_eq!(result, "https://example.com/file.zip");
    }

    #[test]
    fn to_public_download_url_never_exposes_openlist_prefix() {
        let result = to_public_download_url(42, 7, "openlist:/GoogleDrive/abc123/file.zip");
        assert!(
            !result.contains("openlist"),
            "public URL must not expose openlist: prefix, got: {result}"
        );
        assert!(!result.contains("sign="));
        assert!(!result.contains("http://"));
        assert!(!result.contains("https://"));
        assert!(!result.contains("googleusercontent"));
        assert!(!result.contains("drive.google"));
        assert!(!result.contains("Authorization"));
        assert!(!result.contains("Bearer"));
    }

    #[test]
    fn to_public_download_url_never_exposes_local_resource_marker() {
        let result = to_public_download_url(42, 7, "/uploads/resources/resource-9-123.zip");
        assert_eq!(result, "/api/games/42/download-links/7/download");
        assert!(!result.contains("/uploads/resources/"));
    }

    #[test]
    fn public_download_link_response_neutralizes_openlist_platform() {
        let response = to_public_download_link_response(DownloadLinkResponse {
            id: 7,
            game_id: 42,
            platform: "OpenList Archive".to_string(),
            url: "openlist:/GoogleDrive/abc123/file.zip".to_string(),
            extract_code: None,
            password: None,
            file_size: Some("1,024 bytes".to_string()),
            created_at: "2026-06-16T00:00:00Z".to_string(),
            updated_at: "2026-06-16T00:00:00Z".to_string(),
        });

        assert_eq!(response.platform, PUBLIC_OPENLIST_PLATFORM);
        assert_eq!(response.url, "/api/games/42/download-links/7/download");
        assert!(!response.platform.to_ascii_lowercase().contains("openlist"));
        assert!(!response.url.to_ascii_lowercase().contains("openlist"));
    }

    #[test]
    fn public_download_link_response_hides_local_resource_marker() {
        let response = to_public_download_link_response(DownloadLinkResponse {
            id: 7,
            game_id: 42,
            platform: "User upload".to_string(),
            url: "/uploads/resources/resource-9-123.zip".to_string(),
            extract_code: None,
            password: None,
            file_size: Some("1,024 bytes".to_string()),
            created_at: "2026-06-16T00:00:00Z".to_string(),
            updated_at: "2026-06-16T00:00:00Z".to_string(),
        });

        assert_eq!(response.platform, PUBLIC_OPENLIST_PLATFORM);
        assert_eq!(response.url, "/api/games/42/download-links/7/download");
        assert!(!response.url.contains("/uploads/resources/"));
    }

    #[test]
    fn public_download_link_response_preserves_non_openlist_url_but_neutralizes_label() {
        let response = to_public_download_link_response(DownloadLinkResponse {
            id: 8,
            game_id: 42,
            platform: "OpenList Mirror".to_string(),
            url: "https://example.invalid/file.zip".to_string(),
            extract_code: None,
            password: None,
            file_size: None,
            created_at: "2026-06-16T00:00:00Z".to_string(),
            updated_at: "2026-06-16T00:00:00Z".to_string(),
        });

        assert_eq!(response.platform, "本站 Mirror");
        assert_eq!(response.url, "https://example.invalid/file.zip");
        assert!(!response.platform.to_ascii_lowercase().contains("openlist"));
    }

    #[test]
    fn openlist_download_file_name_uses_last_marker_component() {
        let file_name = openlist_download_file_name(
            "openlist:/GoogleDrive/NoneWhiteSite/complete-galgame-archives/023-ensemble-我喜欢的人.tar.zst",
        )
        .expect("valid OpenList marker should expose archive filename");

        assert_eq!(file_name, "023-ensemble-我喜欢的人.tar.zst");
    }

    #[test]
    fn local_resource_file_name_uses_safe_marker_component() {
        let file_name = local_resource_file_name("/uploads/resources/resource-9-123.zip")
            .expect("valid local marker should expose generated filename");

        assert_eq!(file_name, "resource-9-123.zip");
    }

    #[test]
    fn content_disposition_attachment_encodes_utf8_archive_name() {
        let header = content_disposition_attachment("023-ensemble-我喜欢的人.tar.zst")
            .expect("archive filename should produce a safe Content-Disposition header");

        assert!(
            header.contains("filename=\"023-ensemble-.tar.zst\""),
            "ASCII fallback should keep the archive extension, got: {header}"
        );
        assert!(
            header.contains("filename*=UTF-8''023-ensemble-%E6%88%91%E5%96%9C%E6%AC%A2%E7%9A%84%E4%BA%BA.tar.zst"),
            "UTF-8 filename* should preserve the real archive name, got: {header}"
        );
    }

    #[test]
    fn sign_openlist_path_matches_known_vector() {
        let result = sign_openlist_path(
            "/GoogleDrive/Bakappuru/Bakappuru.part1.rar",
            "test-openlist-token",
        )
        .expect("valid token should sign OpenList path");

        assert_eq!(result, "3nvHn3P1jM_PbK7dN5y2hZNQxHVc4YTjGez8eXmMXNg=:0");
    }

    #[test]
    fn sign_openlist_path_rejects_blank_token() {
        let error = sign_openlist_path("/GoogleDrive/Bakappuru/Bakappuru.part1.rar", "  ")
            .expect_err("blank OpenList token should fail closed");

        assert_eq!(error.code, 50000);
    }

    fn openlist_config(resource_upload_dir: &str) -> OpenListConfig {
        OpenListConfig {
            base_url: "https://openlist.example".to_string(),
            token: "openlist-token".to_string(),
            resource_upload_dir: resource_upload_dir.to_string(),
        }
    }

    #[test]
    fn to_x_accel_redirect_target_uses_signed_openlist_p_path() {
        let result = to_x_accel_redirect_target(
            "openlist:/GoogleDrive/Bakappuru/Bakappuru.part1.rar",
            "test-openlist-token",
        )
        .expect("valid OpenList marker should create signed target");

        assert_eq!(
            result,
            "/internal/openlist/p/GoogleDrive/Bakappuru/Bakappuru.part1.rar?sign=3nvHn3P1jM_PbK7dN5y2hZNQxHVc4YTjGez8eXmMXNg=:0"
        );
    }

    #[test]
    fn to_x_accel_redirect_target_encodes_internal_openlist_path() {
        let result = to_x_accel_redirect_target(
            "openlist:/GoogleDrive/ensemble/少女 文件.rar",
            "test-openlist-token",
        )
        .expect("valid OpenList marker should create signed target");

        assert!(result.starts_with("/internal/openlist/p/GoogleDrive/"));
        assert!(result.contains("ensemble/%E5%B0%91%E5%A5%B3%20%E6%96%87%E4%BB%B6.rar"));
        assert!(result.contains("?sign="));
        assert!(result.ends_with(":0"));
        assert!(!result.contains("openlist:"));
        assert!(!result.contains("http://"));
        assert!(!result.contains("https://"));
        assert!(!result.contains("googleusercontent"));
        assert!(!result.contains("drive.google"));
        assert!(!result.contains("Authorization"));
        assert!(!result.contains("Bearer"));
    }
}
