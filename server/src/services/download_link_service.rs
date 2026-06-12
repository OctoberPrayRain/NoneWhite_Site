use sqlx::PgPool;

use crate::{
    dto::download_links::{DownloadLinkRequest, DownloadLinkResponse},
    error::{AppError, AppResult},
    repositories::{download_link_repository, game_repository},
};

pub async fn list_download_links(
    pool: &PgPool,
    game_id: i64,
) -> AppResult<Vec<DownloadLinkResponse>> {
    ensure_game_exists(pool, game_id).await?;

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

fn validate_download_link_request(
    mut request: DownloadLinkRequest,
) -> AppResult<DownloadLinkRequest> {
    request.platform = required_text(request.platform)?;
    request.url = required_text(request.url)?;
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
}
