use serde::{Deserialize, Serialize};

use crate::models::game::{CategoryRow, GameRow, ScreenshotRow, TagRow};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct GameListParams {
    pub page: i64,
    pub page_size: i64,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotResponse {
    pub id: i64,
    pub url: String,
    pub sort_order: i32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    pub id: i64,
    pub title: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: Option<String>,
    pub description: String,
    pub cover_url: String,
    pub category_id: i64,
    pub category: CategoryResponse,
    pub tags: Vec<TagResponse>,
    pub likes_count: i32,
    pub favorites_count: i32,
    pub screenshots: Vec<ScreenshotResponse>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameListResponse {
    pub list: Vec<GameResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

impl From<CategoryRow> for CategoryResponse {
    fn from(category: CategoryRow) -> Self {
        Self {
            id: category.id,
            name: category.name,
            slug: category.slug,
        }
    }
}

impl From<TagRow> for TagResponse {
    fn from(tag: TagRow) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            slug: tag.slug,
        }
    }
}

impl From<ScreenshotRow> for ScreenshotResponse {
    fn from(screenshot: ScreenshotRow) -> Self {
        Self {
            id: screenshot.id,
            url: screenshot.url,
            sort_order: screenshot.sort_order,
        }
    }
}

impl GameResponse {
    pub fn from_parts(
        game: GameRow,
        tags: Vec<TagResponse>,
        screenshots: Vec<ScreenshotResponse>,
    ) -> Self {
        let release_date = game.release_date.map(|date| date.to_string());
        let cover_url = game.cover_url.unwrap_or_default();

        Self {
            id: game.id,
            title: game.title,
            developer: game.developer,
            publisher: game.publisher,
            release_date,
            description: game.description,
            cover_url,
            category_id: game.category_id,
            category: CategoryResponse {
                id: game.category_id,
                name: game.category_name,
                slug: game.category_slug,
            },
            tags,
            likes_count: game.likes_count,
            favorites_count: game.favorites_count,
            screenshots,
        }
    }
}
