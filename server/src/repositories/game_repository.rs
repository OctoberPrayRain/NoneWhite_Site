use sqlx::PgPool;

use crate::{
    dto::games::GameListParams,
    models::game::{CategoryRow, GameRow, ScreenshotRow, TagRow},
};

pub async fn list_categories(pool: &PgPool) -> Result<Vec<CategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, CategoryRow>(
        r#"
        SELECT id, name, slug
        FROM categories
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn list_tags(pool: &PgPool) -> Result<Vec<TagRow>, sqlx::Error> {
    sqlx::query_as::<_, TagRow>(
        r#"
        SELECT id, name, slug
        FROM tags
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn count_games(pool: &PgPool, params: &GameListParams) -> Result<i64, sqlx::Error> {
    let record = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM games g
        WHERE ($1::BIGINT IS NULL OR g.category_id = $1)
          AND (
            $2::BIGINT IS NULL
            OR EXISTS (
              SELECT 1
              FROM game_tags gt
              WHERE gt.game_id = g.id AND gt.tag_id = $2
            )
          )
        "#,
    )
    .bind(params.category_id)
    .bind(params.tag_id)
    .fetch_one(pool)
    .await?;

    Ok(record.0)
}

pub async fn list_games(
    pool: &PgPool,
    params: &GameListParams,
) -> Result<Vec<GameRow>, sqlx::Error> {
    let offset = (params.page - 1) * params.page_size;

    sqlx::query_as::<_, GameRow>(
        r#"
        SELECT
          g.id,
          g.title,
          g.developer,
          g.publisher,
          g.release_date,
          g.description,
          g.cover_url,
          g.category_id,
          c.name AS category_name,
          c.slug AS category_slug,
          g.likes_count,
          g.favorites_count
        FROM games g
        INNER JOIN categories c ON c.id = g.category_id
        WHERE ($1::BIGINT IS NULL OR g.category_id = $1)
          AND (
            $2::BIGINT IS NULL
            OR EXISTS (
              SELECT 1
              FROM game_tags gt
              WHERE gt.game_id = g.id AND gt.tag_id = $2
            )
          )
        ORDER BY g.id ASC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(params.category_id)
    .bind(params.tag_id)
    .bind(params.page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn find_game_by_id(pool: &PgPool, id: i64) -> Result<Option<GameRow>, sqlx::Error> {
    sqlx::query_as::<_, GameRow>(
        r#"
        SELECT
          g.id,
          g.title,
          g.developer,
          g.publisher,
          g.release_date,
          g.description,
          g.cover_url,
          g.category_id,
          c.name AS category_name,
          c.slug AS category_slug,
          g.likes_count,
          g.favorites_count
        FROM games g
        INNER JOIN categories c ON c.id = g.category_id
        WHERE g.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_tags_for_games(
    pool: &PgPool,
    game_ids: &[i64],
) -> Result<Vec<(i64, TagRow)>, sqlx::Error> {
    if game_ids.is_empty() {
        return Ok(Vec::new());
    }

    sqlx::query_as::<_, (i64, i64, String, String)>(
        r#"
        SELECT gt.game_id, t.id, t.name, t.slug
        FROM game_tags gt
        INNER JOIN tags t ON t.id = gt.tag_id
        WHERE gt.game_id = ANY($1)
        ORDER BY gt.game_id ASC, t.id ASC
        "#,
    )
    .bind(game_ids)
    .fetch_all(pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|(game_id, id, name, slug)| (game_id, TagRow { id, name, slug }))
            .collect()
    })
}

pub async fn list_screenshots_for_game(
    pool: &PgPool,
    game_id: i64,
) -> Result<Vec<ScreenshotRow>, sqlx::Error> {
    sqlx::query_as::<_, ScreenshotRow>(
        r#"
        SELECT id, game_id, url, sort_order
        FROM screenshots
        WHERE game_id = $1
        ORDER BY sort_order ASC, id ASC
        "#,
    )
    .bind(game_id)
    .fetch_all(pool)
    .await
}
