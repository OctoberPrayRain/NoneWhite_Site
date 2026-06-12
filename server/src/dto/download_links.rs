use serde::{Deserialize, Serialize};

use crate::models::download_link::DownloadLinkRow;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLinkRequest {
    pub platform: String,
    pub url: String,
    pub extract_code: Option<String>,
    pub password: Option<String>,
    pub file_size: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLinkResponse {
    pub id: i64,
    pub game_id: i64,
    pub platform: String,
    pub url: String,
    pub extract_code: Option<String>,
    pub password: Option<String>,
    pub file_size: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<DownloadLinkRow> for DownloadLinkResponse {
    fn from(row: DownloadLinkRow) -> Self {
        Self {
            id: row.id,
            game_id: row.game_id,
            platform: row.platform,
            url: row.url,
            extract_code: row.extract_code,
            password: row.password,
            file_size: row.file_size,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}
