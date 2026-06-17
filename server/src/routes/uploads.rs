use axum::{
    extract::{Multipart, Path, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use tokio::fs;

use crate::{
    dto::uploads::{ImageUploadResponse, ResourceUploadResponse},
    error::{AppError, AppResult},
    middleware::auth,
    response::ApiResponse,
    services::{openlist_service, upload_service},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/uploads/images", post(upload_image))
        .route("/api/uploads/resources", post(upload_resource))
        .route("/uploads/avatars/{file_name}", get(get_avatar))
        .route("/uploads/images/{file_name}", get(get_image))
}

async fn upload_image(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ImageUploadResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::image_file_required())?
    {
        if field.name() != Some("image") {
            continue;
        }

        let content_type = field.content_type().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::image_file_required())?;
        let extension = upload_service::validate_user_image_file(
            content_type.as_deref(),
            &bytes,
            state.config.upload.max_image_size_bytes,
        )?;
        let image_dir = state.config.upload.upload_dir.join("images");
        fs::create_dir_all(&image_dir)
            .await
            .map_err(|_| AppError::internal())?;

        let file_name = upload_service::stored_file_name("image", user_id, extension)?;
        let file_path = image_dir.join(&file_name);
        fs::write(&file_path, &bytes)
            .await
            .map_err(|_| AppError::internal())?;

        let image_url = format!(
            "{}/images/{}",
            state.config.upload.public_base_url, file_name
        );

        return Ok(Json(ApiResponse::success(
            ImageUploadResponse { image_url },
            "Image uploaded successfully",
        )));
    }

    Err(AppError::image_file_required())
}

async fn upload_resource(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<ResourceUploadResponse>>> {
    let user_id = auth::authenticated_user_id(&headers, &state.config.auth)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::resource_file_required())?
    {
        if field.name() != Some("resource") {
            continue;
        }

        let original_file_name = field.file_name().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::resource_file_required())?;
        upload_service::validate_resource_file(
            &bytes,
            state.config.upload.max_resource_size_bytes,
        )?;
        let file_name = upload_service::stored_resource_file_name(
            "resource",
            user_id,
            original_file_name.as_deref(),
        )?;
        let resource_url =
            openlist_service::resource_upload_marker(&state.config.openlist, &file_name)?;
        openlist_service::upload_resource_bytes(
            &state.http_client,
            &state.config.openlist,
            &resource_url,
            &bytes,
        )
        .await?;
        let response_file_name =
            resource_upload_response_file_name(original_file_name.as_deref(), &file_name);

        return Ok(Json(ApiResponse::success(
            ResourceUploadResponse {
                resource_url,
                file_name: response_file_name,
                file_size: bytes.len(),
            },
            "Resource uploaded successfully",
        )));
    }

    Err(AppError::resource_file_required())
}

async fn get_avatar(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> AppResult<impl IntoResponse> {
    let content_type =
        avatar_content_type(&file_name).ok_or_else(AppError::uploaded_file_not_found)?;
    if !upload_service::is_safe_file_name(&file_name) {
        return Err(AppError::uploaded_file_not_found());
    }

    let file_path = state
        .config
        .upload
        .upload_dir
        .join("avatars")
        .join(file_name);
    let bytes = fs::read(file_path)
        .await
        .map_err(|_| AppError::uploaded_file_not_found())?;

    Ok((
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        bytes,
    ))
}

async fn get_image(
    State(state): State<AppState>,
    Path(file_name): Path<String>,
) -> AppResult<impl IntoResponse> {
    let content_type =
        image_content_type(&file_name).ok_or_else(AppError::uploaded_file_not_found)?;
    if !upload_service::is_safe_file_name(&file_name) {
        return Err(AppError::uploaded_file_not_found());
    }

    let file_path = state
        .config
        .upload
        .upload_dir
        .join("images")
        .join(file_name);
    let bytes = fs::read(file_path)
        .await
        .map_err(|_| AppError::uploaded_file_not_found())?;

    Ok((
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        bytes,
    ))
}

fn resource_upload_response_file_name(
    original_file_name: Option<&str>,
    stored_file_name: &str,
) -> String {
    original_file_name
        .filter(|file_name| !file_name.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| stored_file_name.to_string())
}

fn avatar_content_type(file_name: &str) -> Option<&'static str> {
    image_content_type(file_name)
}

fn image_content_type(file_name: &str) -> Option<&'static str> {
    if file_name.ends_with(".png") {
        Some("image/png")
    } else if file_name.ends_with(".jpg") {
        Some("image/jpeg")
    } else if file_name.ends_with(".webp") {
        Some("image/webp")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Bytes, routing::put, Router};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;
    use serde_json::{json, Value};
    use sqlx::postgres::PgPoolOptions;
    use std::{
        env, fs,
        path::PathBuf,
        sync::{Arc, Mutex},
        time::{SystemTime, UNIX_EPOCH},
    };
    use tokio::{net::TcpListener, task::JoinHandle};

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct CapturedOpenListRequest {
        authorization: Option<String>,
        file_path: Option<String>,
        content_type: Option<String>,
        content_length: Option<String>,
        overwrite: Option<String>,
        body: Vec<u8>,
    }

    #[derive(Serialize)]
    struct TestClaims {
        sub: String,
        exp: usize,
        iat: usize,
    }

    #[test]
    fn is_safe_file_name_rejects_traversal() {
        assert!(!upload_service::is_safe_file_name("../avatar.png"));
        assert!(!upload_service::is_safe_file_name("..avatar.png"));
    }

    #[test]
    fn resource_upload_response_file_name_uses_original_name_when_present() {
        let file_name =
            resource_upload_response_file_name(Some("Original Release.zip"), "resource-9-123.zip");

        assert_eq!(file_name, "Original Release.zip");
    }

    #[test]
    fn resource_upload_response_file_name_falls_back_to_stored_name() {
        let file_name = resource_upload_response_file_name(None, "resource-9-123.zip");

        assert_eq!(file_name, "resource-9-123.zip");
    }

    #[tokio::test]
    async fn resource_upload_route_uploads_to_openlist_without_local_write() {
        let captured = Arc::new(Mutex::new(None));
        let openlist_app = Router::new()
            .route("/api/fs/put", put(capture_openlist_resource_upload))
            .with_state(Arc::clone(&captured));
        let (openlist_base_url, openlist_server) = spawn_test_server(openlist_app).await;
        let upload_dir = unique_temp_dir("resource-route");
        let auth_config = crate::config::AuthConfig {
            jwt_secret: "test-secret".to_string(),
            jwt_expires_in_seconds: 3600,
        };
        let state = AppState {
            config: crate::config::AppConfig {
                server: crate::config::ServerConfig {
                    host: "127.0.0.1".to_string(),
                    port: "0".to_string(),
                },
                database: crate::config::DatabaseConfig {
                    database_url: "postgres://nonewhite:nonewhite@localhost/nonewhite".to_string(),
                },
                auth: auth_config.clone(),
                upload: crate::config::UploadConfig {
                    upload_dir: upload_dir.clone(),
                    public_base_url: "/uploads".to_string(),
                    max_avatar_size_bytes: 1024,
                    max_image_size_bytes: 1024,
                    max_resource_size_bytes: 1024,
                },
                openlist: crate::config::OpenListConfig {
                    base_url: openlist_base_url,
                    token: "openlist-token".to_string(),
                    resource_upload_dir: "openlist:/GoogleDrive/uploads".to_string(),
                },
            },
            db_pool: PgPoolOptions::new()
                .connect_lazy("postgres://nonewhite:nonewhite@localhost/nonewhite")
                .expect("lazy db pool should not connect during upload route test"),
            http_client: reqwest::Client::new(),
        };
        let app = routes().with_state(state);
        let (api_base_url, api_server) = spawn_test_server(app).await;
        let token = test_token(42, &auth_config);
        let response = reqwest::Client::new()
            .post(format!("{api_base_url}/api/uploads/resources"))
            .bearer_auth(token)
            .header(
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=nonewhite-test-boundary",
            )
            .body(resource_multipart_body(
                "nonewhite-test-boundary",
                "Original Release.zip",
                b"route-bytes",
            ))
            .send()
            .await
            .expect("resource upload route request should complete");

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body = response
            .json::<Value>()
            .await
            .expect("resource upload response should be JSON");
        assert_eq!(body["code"], 0);
        assert_eq!(body["data"]["fileName"], "Original Release.zip");
        assert_eq!(body["data"]["fileSize"], 11);
        let resource_url = body["data"]["resourceUrl"]
            .as_str()
            .expect("resource URL should be a string");
        assert!(resource_url.starts_with("openlist:/GoogleDrive/uploads/resource-42-"));
        assert!(resource_url.ends_with(".zip"));
        assert!(!upload_dir.join("resources").exists());

        let stored_file_name = resource_url
            .rsplit('/')
            .next()
            .expect("resource marker should include a file name");
        let captured = captured
            .lock()
            .expect("captured OpenList request lock should not be poisoned")
            .clone()
            .expect("OpenList request should be captured");
        assert_eq!(captured.authorization.as_deref(), Some("openlist-token"));
        assert_eq!(
            captured.file_path.as_deref(),
            Some(format!("%2FGoogleDrive%2Fuploads%2F{stored_file_name}").as_str())
        );
        assert_eq!(
            captured.content_type.as_deref(),
            Some("application/octet-stream")
        );
        assert_eq!(captured.content_length.as_deref(), Some("11"));
        assert_eq!(captured.overwrite.as_deref(), Some("false"));
        assert_eq!(captured.body, b"route-bytes");

        api_server.abort();
        openlist_server.abort();
        fs::remove_dir_all(upload_dir).expect("temporary upload dir should be removable");
    }

    #[test]
    fn avatar_content_type_accepts_whitelisted_extensions() {
        assert_eq!(avatar_content_type("a.png"), Some("image/png"));
        assert_eq!(avatar_content_type("a.jpg"), Some("image/jpeg"));
        assert_eq!(avatar_content_type("a.webp"), Some("image/webp"));
        assert_eq!(avatar_content_type("a.gif"), None);
    }

    #[test]
    fn image_content_type_reuses_image_whitelist() {
        assert_eq!(image_content_type("cover.png"), Some("image/png"));
        assert_eq!(image_content_type("cover.gif"), None);
    }

    async fn capture_openlist_resource_upload(
        State(captured): State<Arc<Mutex<Option<CapturedOpenListRequest>>>>,
        headers: HeaderMap,
        body: Bytes,
    ) -> Json<Value> {
        let request = CapturedOpenListRequest {
            authorization: header_value(&headers, header::AUTHORIZATION.as_str()),
            file_path: header_value(&headers, "File-Path"),
            content_type: header_value(&headers, header::CONTENT_TYPE.as_str()),
            content_length: header_value(&headers, header::CONTENT_LENGTH.as_str()),
            overwrite: header_value(&headers, "Overwrite"),
            body: body.to_vec(),
        };
        *captured
            .lock()
            .expect("captured OpenList request lock should not be poisoned") = Some(request);

        Json(json!({ "code": 200 }))
    }

    async fn spawn_test_server(app: Router) -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("test listener should bind");
        let address = listener
            .local_addr()
            .expect("test listener address should be readable");
        let server = tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("test server should run");
        });

        (format!("http://{address}"), server)
    }

    fn resource_multipart_body(boundary: &str, file_name: &str, bytes: &[u8]) -> Vec<u8> {
        let mut body = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"resource\"; filename=\"{file_name}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .into_bytes();
        body.extend_from_slice(bytes);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        body
    }

    fn test_token(user_id: i64, auth_config: &crate::config::AuthConfig) -> String {
        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_secs() as usize;
        let claims = TestClaims {
            sub: user_id.to_string(),
            iat: issued_at,
            exp: issued_at + auth_config.jwt_expires_in_seconds as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
        )
        .expect("test token should encode")
    }

    fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string)
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let path = env::temp_dir().join(format!("nonewhite-{label}-{}", unique_suffix()));
        fs::create_dir_all(&path).expect("temp upload dir should be creatable");
        path
    }

    fn unique_suffix() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos()
    }
}
