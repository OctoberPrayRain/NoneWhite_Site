use sqlx::{PgPool, Postgres, Transaction};

use crate::{dto::download_links::DownloadLinkRequest, models::download_link::DownloadLinkRow};

pub async fn list_download_links(
    pool: &PgPool,
    game_id: i64,
) -> Result<Vec<DownloadLinkRow>, sqlx::Error> {
    sqlx::query_as::<_, DownloadLinkRow>(
        r#"
        SELECT id, game_id, platform, url, extract_code, password, file_size, created_at, updated_at
        FROM download_links
        WHERE game_id = $1
        ORDER BY id ASC
        "#,
    )
    .bind(game_id)
    .fetch_all(pool)
    .await
}

pub async fn find_download_link(
    pool: &PgPool,
    game_id: i64,
    id: i64,
) -> Result<Option<DownloadLinkRow>, sqlx::Error> {
    sqlx::query_as::<_, DownloadLinkRow>(
        r#"
        SELECT id, game_id, platform, url, extract_code, password, file_size, created_at, updated_at
        FROM download_links
        WHERE game_id = $1 AND id = $2
        "#,
    )
    .bind(game_id)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_download_link(
    pool: &PgPool,
    game_id: i64,
    request: &DownloadLinkRequest,
) -> Result<DownloadLinkRow, sqlx::Error> {
    sqlx::query_as::<_, DownloadLinkRow>(
        r#"
        INSERT INTO download_links (game_id, platform, url, extract_code, password, file_size)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, game_id, platform, url, extract_code, password, file_size, created_at, updated_at
        "#,
    )
    .bind(game_id)
    .bind(&request.platform)
    .bind(&request.url)
    .bind(&request.extract_code)
    .bind(&request.password)
    .bind(&request.file_size)
    .fetch_one(pool)
    .await
}

pub async fn insert_download_link(
    tx: &mut Transaction<'_, Postgres>,
    game_id: i64,
    request: &DownloadLinkRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO download_links (game_id, platform, url, extract_code, password, file_size)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(game_id)
    .bind(&request.platform)
    .bind(&request.url)
    .bind(&request.extract_code)
    .bind(&request.password)
    .bind(&request.file_size)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn update_download_link(
    pool: &PgPool,
    game_id: i64,
    id: i64,
    request: &DownloadLinkRequest,
) -> Result<Option<DownloadLinkRow>, sqlx::Error> {
    sqlx::query_as::<_, DownloadLinkRow>(
        r#"
        UPDATE download_links
        SET platform = $3,
            url = $4,
            extract_code = $5,
            password = $6,
            file_size = $7,
            updated_at = NOW()
        WHERE game_id = $1 AND id = $2
        RETURNING id, game_id, platform, url, extract_code, password, file_size, created_at, updated_at
        "#,
    )
    .bind(game_id)
    .bind(id)
    .bind(&request.platform)
    .bind(&request.url)
    .bind(&request.extract_code)
    .bind(&request.password)
    .bind(&request.file_size)
    .fetch_optional(pool)
    .await
}

pub async fn delete_download_link(
    pool: &PgPool,
    game_id: i64,
    id: i64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM download_links
        WHERE game_id = $1 AND id = $2
        "#,
    )
    .bind(game_id)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
