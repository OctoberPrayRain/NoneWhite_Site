use std::collections::HashMap;

use sqlx::PgPool;

use crate::{
    dto::games::{
        CategoryResponse, GameListParams, GameListQuery, GameListResponse, GameResponse,
        ScreenshotResponse, TagResponse,
    },
    error::{AppError, AppResult},
    models::game::TagRow,
    repositories::game_repository,
};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PAGE_SIZE: i64 = 12;
const MAX_PAGE_SIZE: i64 = 50;

pub async fn list_categories(pool: &PgPool) -> AppResult<Vec<CategoryResponse>> {
    let categories = game_repository::list_categories(pool)
        .await
        .map_err(|_| AppError::internal())?;

    Ok(categories.into_iter().map(CategoryResponse::from).collect())
}

pub async fn list_tags(pool: &PgPool) -> AppResult<Vec<TagResponse>> {
    let tags = game_repository::list_tags(pool)
        .await
        .map_err(|_| AppError::internal())?;

    Ok(tags.into_iter().map(TagResponse::from).collect())
}

pub async fn list_games(pool: &PgPool, query: GameListQuery) -> AppResult<GameListResponse> {
    let params = normalize_query(query);
    let total = game_repository::count_games(pool, &params)
        .await
        .map_err(|_| AppError::internal())?;
    let games = game_repository::list_games(pool, &params)
        .await
        .map_err(|_| AppError::internal())?;
    let game_ids = games.iter().map(|game| game.id).collect::<Vec<_>>();
    let tags_by_game = tags_by_game_id(
        game_repository::list_tags_for_games(pool, &game_ids)
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

pub async fn get_game_detail(pool: &PgPool, id: i64) -> AppResult<GameResponse> {
    if id <= 0 {
        return Err(AppError::game_not_found());
    }

    let game = game_repository::find_game_by_id(pool, id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::game_not_found)?;
    let tags = game_repository::list_tags_for_games(pool, &[id])
        .await
        .map_err(|_| AppError::internal())?
        .into_iter()
        .map(|(_, tag)| TagResponse::from(tag))
        .collect();
    let screenshots = game_repository::list_screenshots_for_game(pool, id)
        .await
        .map_err(|_| AppError::internal())?
        .into_iter()
        .map(ScreenshotResponse::from)
        .collect();

    Ok(GameResponse::from_parts(game, tags, screenshots))
}

fn normalize_query(query: GameListQuery) -> GameListParams {
    let page = query.page.filter(|page| *page > 0).unwrap_or(DEFAULT_PAGE);
    let page_size = query
        .page_size
        .filter(|page_size| *page_size > 0)
        .map(|page_size| page_size.min(MAX_PAGE_SIZE))
        .unwrap_or(DEFAULT_PAGE_SIZE);
    let category_id = query.category_id.filter(|category_id| *category_id > 0);
    let tag_id = query.tag_id.filter(|tag_id| *tag_id > 0);

    GameListParams {
        page,
        page_size,
        category_id,
        tag_id,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_defaults_invalid_pagination() {
        let params = normalize_query(GameListQuery {
            page: Some(0),
            page_size: Some(-1),
            category_id: Some(-3),
            tag_id: Some(0),
        });

        assert_eq!(params.page, DEFAULT_PAGE);
        assert_eq!(params.page_size, DEFAULT_PAGE_SIZE);
        assert_eq!(params.category_id, None);
        assert_eq!(params.tag_id, None);
    }

    #[test]
    fn normalize_query_caps_page_size() {
        let params = normalize_query(GameListQuery {
            page: Some(2),
            page_size: Some(999),
            category_id: Some(1),
            tag_id: Some(2),
        });

        assert_eq!(params.page, 2);
        assert_eq!(params.page_size, MAX_PAGE_SIZE);
        assert_eq!(params.category_id, Some(1));
        assert_eq!(params.tag_id, Some(2));
    }
}
