use sqlx::types::chrono::NaiveDate;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct CategoryRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TagRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ScreenshotRow {
    pub id: i64,
    pub game_id: i64,
    pub url: String,
    pub sort_order: i32,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct GameRow {
    pub id: i64,
    pub title: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: Option<NaiveDate>,
    pub description: String,
    pub cover_url: Option<String>,
    pub category_id: i64,
    pub category_name: String,
    pub category_slug: String,
    pub likes_count: i32,
    pub favorites_count: i32,
}
