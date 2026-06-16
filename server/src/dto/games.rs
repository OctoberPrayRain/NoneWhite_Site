use serde::{Deserialize, Serialize};

use crate::models::game::{CategoryRow, GameRow, ScreenshotRow, TagRow};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GameListParams {
    pub page: i64,
    pub page_size: i64,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
    pub keyword: Option<String>,
    pub approval_status: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameScreenshotRequest {
    pub url: String,
    pub sort_order: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameRequest {
    pub title: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: Option<String>,
    pub description: String,
    pub cover_url: Option<String>,
    pub category_id: i64,
    pub tag_ids: Vec<i64>,
    pub screenshots: Vec<GameScreenshotRequest>,
}

pub type UpdateGameRequest = CreateGameRequest;

#[derive(Clone, Debug)]
pub struct ValidatedGameInput {
    pub title: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: Option<sqlx::types::chrono::NaiveDate>,
    pub description: String,
    pub cover_url: Option<String>,
    pub category_id: i64,
    pub tag_ids: Vec<i64>,
    pub screenshots: Vec<ValidatedScreenshotInput>,
}

#[derive(Clone, Debug)]
pub struct ValidatedScreenshotInput {
    pub url: String,
    pub sort_order: i32,
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
    pub approval_status: String,
    pub submitted_by_user_id: Option<i64>,
    pub reviewed_by_user_id: Option<i64>,
    pub reviewed_at: Option<String>,
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
        let reviewed_at = game.reviewed_at.map(|date| date.to_rfc3339());

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
            approval_status: game.approval_status,
            submitted_by_user_id: game.submitted_by_user_id,
            reviewed_by_user_id: game.reviewed_by_user_id,
            reviewed_at,
            screenshots,
        }
    }

    pub fn public_from_parts(
        game: GameRow,
        tags: Vec<TagResponse>,
        screenshots: Vec<ScreenshotResponse>,
    ) -> Self {
        let mut response = Self::from_parts(game, tags, screenshots);
        response.developer = neutralize_public_storage_text(&response.developer);
        response.publisher = neutralize_public_storage_text(&response.publisher);
        response.description = neutralize_public_storage_text(&response.description);
        response
    }
}

fn neutralize_public_storage_text(value: &str) -> String {
    value
        .replace("OpenList 托管", "本站托管")
        .replace("OpenList Library", "本站资料库")
        .replace("OpenList", "本站")
        .replace("OPENLIST", "本站")
        .replace("openlist", "本站")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_row() -> GameRow {
        GameRow {
            id: 101,
            title: "Bakappuru".to_string(),
            developer: "OpenList Studio".to_string(),
            publisher: "OpenList Library".to_string(),
            release_date: None,
            description: "OpenList 托管的 Bakappuru 游戏压缩包资源，按分卷提供下载。".to_string(),
            cover_url: None,
            category_id: 3,
            category_name: "恋爱".to_string(),
            category_slug: "romance".to_string(),
            likes_count: 0,
            favorites_count: 0,
            approval_status: "approved".to_string(),
            submitted_by_user_id: None,
            reviewed_by_user_id: None,
            reviewed_at: None,
        }
    }

    #[test]
    fn public_game_response_neutralizes_openlist_storage_labels() {
        let response = GameResponse::public_from_parts(game_row(), Vec::new(), Vec::new());
        let public_text = format!(
            "{} {} {}",
            response.developer, response.publisher, response.description
        );

        assert_eq!(response.publisher, "本站资料库");
        assert_eq!(
            response.description,
            "本站托管的 Bakappuru 游戏压缩包资源，按分卷提供下载。"
        );
        assert!(!public_text.to_ascii_lowercase().contains("openlist"));
    }

    #[test]
    fn raw_game_response_preserves_admin_storage_labels() {
        let response = GameResponse::from_parts(game_row(), Vec::new(), Vec::new());

        assert_eq!(response.publisher, "OpenList Library");
        assert!(response.description.contains("OpenList"));
    }
}
