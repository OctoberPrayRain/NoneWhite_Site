use std::collections::HashMap;

use sqlx::{types::chrono::NaiveDate, PgPool};

use crate::{
    dto::games::{
        CategoryResponse, CreateGameRequest, GameListParams, GameListQuery, GameListResponse,
        GameResponse, ScreenshotResponse, TagResponse, UpdateGameRequest, ValidatedGameInput,
        ValidatedScreenshotInput,
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

pub async fn create_game(pool: &PgPool, request: CreateGameRequest) -> AppResult<GameResponse> {
    let input = validate_game_input(request)?;
    ensure_category_exists(pool, input.category_id).await?;
    ensure_tags_exist(pool, &input.tag_ids).await?;

    let search_text = game_repository::build_search_text(
        &input.title,
        &input.developer,
        &input.publisher,
        &input.description,
    );
    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    let game_id = game_repository::insert_game(&mut tx, &input, &search_text)
        .await
        .map_err(map_game_write_error)?;
    game_repository::replace_game_tags(&mut tx, game_id, &input.tag_ids)
        .await
        .map_err(map_game_write_error)?;
    game_repository::replace_screenshots(&mut tx, game_id, &input.screenshots)
        .await
        .map_err(map_game_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    get_game_detail(pool, game_id).await
}

pub async fn update_game(
    pool: &PgPool,
    game_id: i64,
    request: UpdateGameRequest,
) -> AppResult<GameResponse> {
    if game_id <= 0 {
        return Err(AppError::game_not_found());
    }
    ensure_game_exists(pool, game_id).await?;

    let input = validate_game_input(request)?;
    ensure_category_exists(pool, input.category_id).await?;
    ensure_tags_exist(pool, &input.tag_ids).await?;

    let search_text = game_repository::build_search_text(
        &input.title,
        &input.developer,
        &input.publisher,
        &input.description,
    );
    let mut tx = pool.begin().await.map_err(|_| AppError::internal())?;
    game_repository::update_game(&mut tx, game_id, &input, &search_text)
        .await
        .map_err(map_game_write_error)?;
    game_repository::replace_game_tags(&mut tx, game_id, &input.tag_ids)
        .await
        .map_err(map_game_write_error)?;
    game_repository::replace_screenshots(&mut tx, game_id, &input.screenshots)
        .await
        .map_err(map_game_write_error)?;
    tx.commit().await.map_err(|_| AppError::internal())?;

    get_game_detail(pool, game_id).await
}

pub async fn delete_game(pool: &PgPool, game_id: i64) -> AppResult<()> {
    if game_id <= 0 {
        return Err(AppError::game_not_found());
    }

    let rows_affected = game_repository::delete_game(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?;
    if rows_affected == 0 {
        return Err(AppError::game_not_found());
    }

    Ok(())
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

fn validate_game_input(request: CreateGameRequest) -> AppResult<ValidatedGameInput> {
    let title = required_text(request.title)?;
    let developer = required_text(request.developer)?;
    let publisher = required_text(request.publisher)?;
    let description = required_text(request.description)?;
    let cover_url = optional_text(request.cover_url);
    let release_date = parse_release_date(request.release_date)?;
    let category_id = if request.category_id > 0 {
        request.category_id
    } else {
        return Err(AppError::category_not_found());
    };
    let tag_ids = normalize_tag_ids(request.tag_ids)?;
    let screenshots = request
        .screenshots
        .into_iter()
        .enumerate()
        .map(|(index, screenshot)| {
            let sort_order = screenshot.sort_order.unwrap_or(index as i32);
            if sort_order < 0 {
                return Err(AppError::game_field_invalid());
            }

            Ok(ValidatedScreenshotInput {
                url: required_text(screenshot.url)?,
                sort_order,
            })
        })
        .collect::<AppResult<Vec<_>>>()?;

    Ok(ValidatedGameInput {
        title,
        developer,
        publisher,
        release_date,
        description,
        cover_url,
        category_id,
        tag_ids,
        screenshots,
    })
}

fn required_text(value: String) -> AppResult<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(AppError::game_field_invalid());
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

fn parse_release_date(value: Option<String>) -> AppResult<Option<NaiveDate>> {
    optional_text(value)
        .map(|value| NaiveDate::parse_from_str(&value, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| AppError::game_field_invalid())
}

fn normalize_tag_ids(tag_ids: Vec<i64>) -> AppResult<Vec<i64>> {
    let mut tag_ids = tag_ids
        .into_iter()
        .filter(|tag_id| *tag_id > 0)
        .collect::<Vec<_>>();
    tag_ids.sort_unstable();
    tag_ids.dedup();

    if tag_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(tag_ids)
}

async fn ensure_game_exists(pool: &PgPool, game_id: i64) -> AppResult<()> {
    game_repository::find_game_by_id(pool, game_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::game_not_found)?;

    Ok(())
}

async fn ensure_category_exists(pool: &PgPool, category_id: i64) -> AppResult<()> {
    let exists = game_repository::category_exists(pool, category_id)
        .await
        .map_err(|_| AppError::internal())?;
    if !exists {
        return Err(AppError::category_not_found());
    }

    Ok(())
}

async fn ensure_tags_exist(pool: &PgPool, tag_ids: &[i64]) -> AppResult<()> {
    let existing_count = game_repository::count_existing_tags(pool, tag_ids)
        .await
        .map_err(|_| AppError::internal())?;
    if existing_count != tag_ids.len() as i64 {
        return Err(AppError::tag_not_found());
    }

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

fn map_game_write_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23503") =>
        {
            AppError::game_field_invalid()
        }
        _ => AppError::internal(),
    }
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

    #[test]
    fn validate_game_input_trims_and_parses_release_date() {
        let input = validate_game_input(CreateGameRequest {
            title: "  Title  ".to_string(),
            developer: " Dev ".to_string(),
            publisher: " Pub ".to_string(),
            release_date: Some("2026-06-12".to_string()),
            description: " Desc ".to_string(),
            cover_url: Some("  /uploads/images/cover.png ".to_string()),
            category_id: 1,
            tag_ids: vec![2, 2, 1],
            screenshots: vec![],
        })
        .expect("valid game input should pass");

        assert_eq!(input.title, "Title");
        assert_eq!(input.tag_ids, vec![1, 2]);
        assert_eq!(
            input.release_date.map(|date| date.to_string()).as_deref(),
            Some("2026-06-12")
        );
    }

    #[test]
    fn validate_game_input_rejects_blank_title() {
        let error = validate_game_input(CreateGameRequest {
            title: "  ".to_string(),
            developer: "Dev".to_string(),
            publisher: "Pub".to_string(),
            release_date: None,
            description: "Desc".to_string(),
            cover_url: None,
            category_id: 1,
            tag_ids: vec![],
            screenshots: vec![],
        })
        .expect_err("blank title should fail");

        assert_eq!(error.code, 40014);
    }
}
