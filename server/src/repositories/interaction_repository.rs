use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    dto::interactions::InteractionListParams,
    models::{
        game::{GameRow, TagRow},
        interaction::{CommentOwnerRow, CommentRow, ParentCommentRow},
    },
};

const COUNT_USER_FAVORITES_SQL: &str = r#"
        SELECT COUNT(*)
        FROM favorites f
        INNER JOIN games g ON g.id = f.game_id
        WHERE f.user_id = $1
          AND g.approval_status = 'approved'
        "#;

const LIST_USER_FAVORITES_SQL: &str = r#"
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
          g.favorites_count,
          g.approval_status,
          g.submitted_by_user_id,
          g.reviewed_by_user_id,
          g.reviewed_at
        FROM favorites f
        INNER JOIN games g ON g.id = f.game_id
        INNER JOIN categories c ON c.id = g.category_id
        WHERE f.user_id = $1
          AND g.approval_status = 'approved'
        ORDER BY f.created_at DESC, g.id DESC
        LIMIT $2 OFFSET $3
        "#;

pub async fn count_comments(pool: &PgPool, game_id: i64) -> Result<i64, sqlx::Error> {
    let record = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM comments
        WHERE game_id = $1
        "#,
    )
    .bind(game_id)
    .fetch_one(pool)
    .await?;

    Ok(record.0)
}

pub async fn list_comments(
    pool: &PgPool,
    game_id: i64,
    params: &InteractionListParams,
) -> Result<Vec<CommentRow>, sqlx::Error> {
    let offset = (params.page - 1) * params.page_size;

    sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT
          c.id,
          c.user_id,
          u.username,
          u.avatar_url,
          c.game_id,
          c.content,
          c.parent_id,
          c.created_at
        FROM comments c
        INNER JOIN users u ON u.id = c.user_id
        WHERE c.game_id = $1
        ORDER BY c.created_at DESC, c.id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(game_id)
    .bind(params.page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn find_parent_comment(
    pool: &PgPool,
    parent_id: i64,
) -> Result<Option<ParentCommentRow>, sqlx::Error> {
    sqlx::query_as::<_, ParentCommentRow>(
        r#"
        SELECT game_id
        FROM comments
        WHERE id = $1
        "#,
    )
    .bind(parent_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_comment(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
    content: &str,
    parent_id: Option<i64>,
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"
        INSERT INTO comments (user_id, game_id, content, parent_id)
        VALUES ($1, $2, $3, $4)
        RETURNING
          id,
          user_id,
          (SELECT username FROM users WHERE id = $1) AS username,
          (SELECT avatar_url FROM users WHERE id = $1) AS avatar_url,
          game_id,
          content,
          parent_id,
          created_at
        "#,
    )
    .bind(user_id)
    .bind(game_id)
    .bind(content)
    .bind(parent_id)
    .fetch_one(pool)
    .await
}

pub async fn find_comment_owner(
    pool: &PgPool,
    id: i64,
) -> Result<Option<CommentOwnerRow>, sqlx::Error> {
    sqlx::query_as::<_, CommentOwnerRow>(
        r#"
        SELECT user_id
        FROM comments
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_comment(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM comments
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_like(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    game_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO likes (user_id, game_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, game_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(game_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn delete_like(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    game_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM likes
        WHERE user_id = $1 AND game_id = $2
        "#,
    )
    .bind(user_id)
    .bind(game_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn refresh_likes_count(
    tx: &mut Transaction<'_, Postgres>,
    game_id: i64,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query_as::<_, (i32,)>(
        r#"
        UPDATE games
        SET likes_count = (
          SELECT COUNT(*)::INTEGER
          FROM likes
          WHERE game_id = $1
        )
        WHERE id = $1
        RETURNING likes_count
        "#,
    )
    .bind(game_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(record.0)
}

pub async fn insert_favorite(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    game_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO favorites (user_id, game_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, game_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(game_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn delete_favorite(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    game_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM favorites
        WHERE user_id = $1 AND game_id = $2
        "#,
    )
    .bind(user_id)
    .bind(game_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn refresh_favorites_count(
    tx: &mut Transaction<'_, Postgres>,
    game_id: i64,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query_as::<_, (i32,)>(
        r#"
        UPDATE games
        SET favorites_count = (
          SELECT COUNT(*)::INTEGER
          FROM favorites
          WHERE game_id = $1
        )
        WHERE id = $1
        RETURNING favorites_count
        "#,
    )
    .bind(game_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(record.0)
}

pub async fn count_user_favorites(pool: &PgPool, user_id: i64) -> Result<i64, sqlx::Error> {
    let record = sqlx::query_as::<_, (i64,)>(COUNT_USER_FAVORITES_SQL)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(record.0)
}

pub async fn list_user_favorites(
    pool: &PgPool,
    user_id: i64,
    params: &InteractionListParams,
) -> Result<Vec<GameRow>, sqlx::Error> {
    let offset = (params.page - 1) * params.page_size;

    sqlx::query_as::<_, GameRow>(LIST_USER_FAVORITES_SQL)
        .bind(user_id)
        .bind(params.page_size)
        .bind(offset)
        .fetch_all(pool)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized_sql(sql: &str) -> String {
        sql.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn count_user_favorites_sql_filters_to_approved_games() {
        let sql = normalized_sql(COUNT_USER_FAVORITES_SQL);

        assert!(sql.contains("INNER JOIN games g ON g.id = f.game_id"));
        assert!(sql.contains("g.approval_status = 'approved'"));
    }

    #[test]
    fn list_user_favorites_sql_filters_to_approved_games() {
        let sql = normalized_sql(LIST_USER_FAVORITES_SQL);

        assert!(sql.contains("INNER JOIN games g ON g.id = f.game_id"));
        assert!(sql.contains("g.approval_status = 'approved'"));
    }

    #[test]
    fn list_user_favorites_sql_selects_current_game_row_moderation_fields() {
        let sql = normalized_sql(LIST_USER_FAVORITES_SQL);

        assert!(sql.contains(
            "g.favorites_count, g.approval_status, g.submitted_by_user_id, g.reviewed_by_user_id, g.reviewed_at FROM favorites f"
        ));
    }
}
