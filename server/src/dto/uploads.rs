use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageUploadResponse {
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUploadResponse {
    pub resource_url: String,
    pub file_name: String,
    pub file_size: usize,
}
