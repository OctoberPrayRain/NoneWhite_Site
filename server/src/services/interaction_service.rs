use std::collections::HashMap;

use sqlx::PgPool;

use crate::{
    dto::{
        games::{GameListResponse, GameResponse, TagResponse},
        interactions::{
            CommentListQuery, CommentListResponse, CommentResponse, CreateCommentRequest,
            FavoriteStatusResponse, InteractionListParams, LikeStatusResponse,
        },
    },
    error::{AppError, AppResult},
    models::game::TagRow,
    repositories::{game_repository, interaction_repository, user_repository},
};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PAGE_SIZE: i64 = 12;
const MAX_PAGE_SIZE: i64 = 50;
const MAX_COMMENT_CONTENT_CHARS: usize = 1000;

pub async fn list_comments(
    pool: &PgPool,
    game_id: i64,
    query: CommentListQuery,
) -> AppResult<CommentListResponse> {
    ensure_game_exists(pool, game_id).await?;

    let params = normalize_pagination(query.page, query.page_size);
    let total = interaction_repository::count_comments(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?;
    let comments = interaction_repository::list_comments(pool, game_id, &params)
        .await
        .map_err(|_| AppError::internal())?
        .into_iter()
        .map(CommentResponse::from)
        .collect();

    Ok(CommentListResponse {
        list: comments,
        total,
        page: params.page,
        page_size: params.page_size,
    })
}

pub async fn create_comment(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
    request: CreateCommentRequest,
) -> AppResult<CommentResponse> {
    ensure_game_exists(pool, game_id).await?;
    ensure_user_exists(pool, user_id).await?;
    let content = validate_comment_content(&request.content)?;

    if let Some(parent_id) = request.parent_id {
        let parent = interaction_repository::find_parent_comment(pool, parent_id)
            .await
            .map_err(|_| AppError::internal())?
            .ok_or_else(AppError::comment_not_found)?;

        if parent.game_id != game_id {
            return Err(AppError::comment_not_found());
        }
    }

    let comment = interaction_repository::create_comment(
        pool,
        user_id,
        game_id,
        content.as_str(),
        request.parent_id,
    )
    .await
    .map_err(map_create_comment_error)?;

    Ok(CommentResponse::from(comment))
}

pub async fn delete_comment(pool: &PgPool, user_id: i64, id: i64) -> AppResult<()> {
    let user = user_repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::user_not_found)?;
    let owner = interaction_repository::find_comment_owner(pool, id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::comment_not_found)?;

    if user.role != "admin" && owner.user_id != user_id {
        return Err(AppError::permission_denied());
    }

    interaction_repository::delete_comment(pool, id)
        .await
        .map_err(|_| AppError::internal())
}

pub async fn like_game(pool: &PgPool, user_id: i64, game_id: i64) -> AppResult<LikeStatusResponse> {
    ensure_game_exists(pool, game_id).await?;
    ensure_user_exists(pool, user_id).await?;

    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    interaction_repository::insert_like(&mut tx, user_id, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    let likes_count = interaction_repository::refresh_likes_count(&mut tx, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    Ok(LikeStatusResponse {
        liked: true,
        likes_count,
    })
}

pub async fn unlike_game(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
) -> AppResult<LikeStatusResponse> {
    ensure_game_exists(pool, game_id).await?;
    ensure_user_exists(pool, user_id).await?;

    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    interaction_repository::delete_like(&mut tx, user_id, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    let likes_count = interaction_repository::refresh_likes_count(&mut tx, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    Ok(LikeStatusResponse {
        liked: false,
        likes_count,
    })
}

pub async fn favorite_game(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
) -> AppResult<FavoriteStatusResponse> {
    ensure_game_exists(pool, game_id).await?;
    ensure_user_exists(pool, user_id).await?;

    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    interaction_repository::insert_favorite(&mut tx, user_id, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    let favorites_count = interaction_repository::refresh_favorites_count(&mut tx, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    Ok(FavoriteStatusResponse {
        favorited: true,
        favorites_count,
    })
}

pub async fn unfavorite_game(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
) -> AppResult<FavoriteStatusResponse> {
    ensure_game_exists(pool, game_id).await?;
    ensure_user_exists(pool, user_id).await?;

    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    interaction_repository::delete_favorite(&mut tx, user_id, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    let favorites_count = interaction_repository::refresh_favorites_count(&mut tx, game_id)
        .await
        .map_err(map_interaction_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    Ok(FavoriteStatusResponse {
        favorited: false,
        favorites_count,
    })
}

pub async fn list_my_favorites(
    pool: &PgPool,
    user_id: i64,
    query: CommentListQuery,
) -> AppResult<GameListResponse> {
    ensure_user_exists(pool, user_id).await?;

    let params = normalize_pagination(query.page, query.page_size);
    let total = interaction_repository::count_user_favorites(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?;
    let games = interaction_repository::list_user_favorites(pool, user_id, &params)
        .await
        .map_err(|_| AppError::internal())?;
    let game_ids = games.iter().map(|game| game.id).collect::<Vec<_>>();
    let tags_by_game = tags_by_game_id(
        interaction_repository::list_tags_for_games(pool, &game_ids)
            .await
            .map_err(|_| AppError::internal())?,
    );

    let list = games
        .into_iter()
        .map(|game| {
            let tags = tags_by_game.get(&game.id).cloned().unwrap_or_default();
            GameResponse::from_parts(game, tags, Vec::new())
        })
        .collect();

    Ok(GameListResponse {
        list,
        total,
        page: params.page,
        page_size: params.page_size,
    })
}

pub(crate) fn normalize_pagination(
    page: Option<i64>,
    page_size: Option<i64>,
) -> InteractionListParams {
    let page = page.filter(|page| *page > 0).unwrap_or(DEFAULT_PAGE);
    let page_size = page_size
        .filter(|page_size| *page_size > 0)
        .map(|page_size| page_size.min(MAX_PAGE_SIZE))
        .unwrap_or(DEFAULT_PAGE_SIZE);

    InteractionListParams { page, page_size }
}

pub(crate) fn validate_comment_content(content: &str) -> AppResult<String> {
    let content = content.trim();

    if content.is_empty() {
        return Err(AppError::comment_content_required());
    }

    if content.chars().count() > MAX_COMMENT_CONTENT_CHARS {
        return Err(AppError::comment_content_too_long());
    }

    Ok(content.to_string())
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

async fn ensure_user_exists(pool: &PgPool, user_id: i64) -> AppResult<()> {
    user_repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::user_not_found)?;

    Ok(())
}

fn tags_by_game_id(rows: Vec<(i64, TagRow)>) -> HashMap<i64, Vec<TagResponse>> {
    let mut tags_by_game: HashMap<i64, Vec<TagResponse>> = HashMap::new();

    for (game_id, tag) in rows {
        tags_by_game
            .entry(game_id)
            .or_default()
            .push(TagResponse::from(tag));
    }

    tags_by_game
}

fn map_create_comment_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23503") =>
        {
            AppError::game_not_found()
        }
        _ => AppError::internal(),
    }
}

fn map_interaction_write_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::RowNotFound => AppError::game_not_found(),
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
    fn validate_comment_content_trims_valid_content() {
        let content = validate_comment_content("  hello interaction  ")
            .expect("trimmed non-empty content should pass");

        assert_eq!(content, "hello interaction");
    }

    #[test]
    fn validate_comment_content_rejects_blank_content() {
        let error =
            validate_comment_content(" \n\t ").expect_err("blank content should be rejected");

        assert_eq!(error.code, 40009);
    }

    #[test]
    fn validate_comment_content_rejects_too_long_content() {
        let content = "好".repeat(MAX_COMMENT_CONTENT_CHARS + 1);
        let error = validate_comment_content(&content)
            .expect_err("content over character limit should be rejected");

        assert_eq!(error.code, 40010);
    }

    #[test]
    fn normalize_pagination_defaults_invalid_values() {
        let params = normalize_pagination(Some(0), Some(-5));

        assert_eq!(params.page, DEFAULT_PAGE);
        assert_eq!(params.page_size, DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn normalize_pagination_caps_page_size() {
        let params = normalize_pagination(Some(3), Some(999));

        assert_eq!(params.page, 3);
        assert_eq!(params.page_size, MAX_PAGE_SIZE);
    }
}
