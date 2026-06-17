use reqwest::{header, Client, Method, Request, StatusCode};
use serde::Deserialize;
use std::time::Duration;

use crate::{
    config::OpenListConfig,
    error::{AppError, AppResult},
};

const OPENLIST_SCHEME: &str = "openlist:";
const OPENLIST_GOOGLE_DRIVE_PREFIX: &str = "openlist:/GoogleDrive/";
const OPENLIST_PUT_PATH: &str = "/api/fs/put";
const FILE_PATH_HEADER: &str = "File-Path";
const OVERWRITE_HEADER: &str = "Overwrite";
const OPENLIST_SUCCESS_CODE: i64 = 200;
const OPENLIST_UPLOAD_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Deserialize)]
struct OpenListPutResponse {
    code: i64,
}

pub(crate) fn resource_upload_marker(
    openlist_config: &OpenListConfig,
    stored_file_name: &str,
) -> AppResult<String> {
    let resource_upload_dir = validate_resource_upload_dir(&openlist_config.resource_upload_dir)?;
    let marker = format!("{resource_upload_dir}/{stored_file_name}");
    if is_valid_openlist_marker(&marker) {
        Ok(marker)
    } else {
        Err(AppError::resource_upload_failed())
    }
}

pub(crate) async fn upload_resource_bytes(
    client: &Client,
    openlist_config: &OpenListConfig,
    resource_marker: &str,
    bytes: &[u8],
) -> AppResult<()> {
    let request = build_put_request(client, openlist_config, resource_marker, bytes)?;
    let response = client
        .execute(request)
        .await
        .map_err(|_| AppError::resource_upload_failed())?;
    let status = response.status();
    let body = response
        .json::<OpenListPutResponse>()
        .await
        .map_err(|_| AppError::resource_upload_failed())?;

    if openlist_upload_succeeded(status, &body) {
        Ok(())
    } else {
        Err(AppError::resource_upload_failed())
    }
}

fn build_put_request(
    client: &Client,
    openlist_config: &OpenListConfig,
    resource_marker: &str,
    bytes: &[u8],
) -> AppResult<Request> {
    let endpoint_url = openlist_put_url(&openlist_config.base_url)?;
    let token = openlist_token(&openlist_config.token)?;
    let remote_path = remote_upload_path(resource_marker)?;

    client
        .request(Method::PUT, endpoint_url)
        .timeout(OPENLIST_UPLOAD_TIMEOUT)
        .header(header::AUTHORIZATION, token)
        .header(FILE_PATH_HEADER, encode_file_path_header(remote_path))
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, bytes.len().to_string())
        .header(OVERWRITE_HEADER, "false")
        .body(bytes.to_vec())
        .build()
        .map_err(|_| AppError::resource_upload_failed())
}

fn openlist_put_url(base_url: &str) -> AppResult<String> {
    let base_url = base_url.trim().trim_end_matches('/');
    if base_url.is_empty() {
        return Err(AppError::resource_upload_failed());
    }

    Ok(format!("{base_url}{OPENLIST_PUT_PATH}"))
}

fn openlist_token(token: &str) -> AppResult<&str> {
    let token = token.trim();
    if token.is_empty() {
        return Err(AppError::resource_upload_failed());
    }

    Ok(token)
}

fn validate_resource_upload_dir(value: &str) -> AppResult<&str> {
    let value = value.trim();
    if is_valid_openlist_marker(value) {
        Ok(value)
    } else {
        Err(AppError::resource_upload_failed())
    }
}

fn remote_upload_path(resource_marker: &str) -> AppResult<&str> {
    let resource_marker = resource_marker.trim();
    if !is_valid_openlist_marker(resource_marker) {
        return Err(AppError::resource_upload_failed());
    }

    resource_marker
        .strip_prefix(OPENLIST_SCHEME)
        .ok_or_else(AppError::resource_upload_failed)
}

fn is_valid_openlist_marker(value: &str) -> bool {
    if value.is_empty()
        || value.chars().any(char::is_control)
        || value.contains('?')
        || value.contains('#')
        || value.contains('\\')
        || value.contains('%')
        || !value.starts_with(OPENLIST_GOOGLE_DRIVE_PREFIX)
    {
        return false;
    }

    let resource_path = &value[OPENLIST_GOOGLE_DRIVE_PREFIX.len()..];
    !resource_path.is_empty()
        && resource_path
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."))
}

fn encode_file_path_header(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(char::from(byte));
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn openlist_upload_succeeded(status: StatusCode, body: &OpenListPutResponse) -> bool {
    status.is_success() && body.code == OPENLIST_SUCCESS_CODE
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Bytes, extract::State, http::HeaderMap, routing::put, Json, Router};
    use serde_json::{json, Value};
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct CapturedRequest {
        authorization: Option<String>,
        file_path: Option<String>,
        content_type: Option<String>,
        content_length: Option<String>,
        overwrite: Option<String>,
        body: Vec<u8>,
    }

    #[test]
    fn resource_upload_marker_appends_stored_file_name_under_configured_dir() {
        let config = openlist_config(
            "https://openlist.example",
            "token",
            "openlist:/GoogleDrive/NoneWhite/uploads",
        );

        let marker = resource_upload_marker(&config, "resource-9-123.zip")
            .expect("upload marker should be generated");

        assert_eq!(
            marker,
            "openlist:/GoogleDrive/NoneWhite/uploads/resource-9-123.zip"
        );
    }

    #[test]
    fn resource_upload_marker_rejects_missing_or_unsafe_configured_dir() {
        let invalid_dirs = [
            "",
            "openlist:/GoogleDrive/",
            "openlist:/GoogleDrive/uploads/",
            "openlist:/GoogleDrive/uploads//resources",
            "openlist:/GoogleDrive/uploads/../secret",
            "openlist:/GoogleDrive/uploads/%2e%2e",
            "openlist:/GoogleDrive/uploads?token=secret",
            "openlist:/GoogleDrive/uploads#fragment",
            "openlist:/GoogleDrive/uploads\\secret",
            "openlist:/GoogleDrive/uploads\u{0000}secret",
            "openlist:/OtherDrive/uploads",
            "/GoogleDrive/uploads",
        ];

        for resource_upload_dir in invalid_dirs {
            let config = openlist_config("https://openlist.example", "token", resource_upload_dir);
            let error = resource_upload_marker(&config, "resource-9-123.zip")
                .expect_err("unsafe upload dir should fail closed");

            assert_eq!(
                error.code, 50201,
                "dir should be rejected: {resource_upload_dir:?}"
            );
        }
    }

    #[test]
    fn remote_upload_path_strips_openlist_scheme() {
        let path = remote_upload_path("openlist:/GoogleDrive/uploads/resource-9-123.zip")
            .expect("valid marker should produce a remote path");

        assert_eq!(path, "/GoogleDrive/uploads/resource-9-123.zip");
    }

    #[test]
    fn file_path_header_percent_encodes_full_remote_path() {
        let encoded = encode_file_path_header("/GoogleDrive/Resource Uploads/file name.zip");

        assert_eq!(
            encoded,
            "%2FGoogleDrive%2FResource%20Uploads%2Ffile%20name.zip"
        );
    }

    #[test]
    fn build_put_request_sets_openlist_contract_headers() {
        let client = Client::new();
        let config = openlist_config(
            "https://openlist.example/",
            "secret-token",
            "openlist:/GoogleDrive/uploads",
        );

        let request = build_put_request(
            &client,
            &config,
            "openlist:/GoogleDrive/Resource Uploads/file name.zip",
            b"abc",
        )
        .expect("request should be buildable");

        assert_eq!(request.method(), Method::PUT);
        assert_eq!(
            request.url().as_str(),
            "https://openlist.example/api/fs/put"
        );
        assert_eq!(
            request.headers().get(header::AUTHORIZATION).unwrap(),
            "secret-token"
        );
        assert_eq!(
            request.headers().get(FILE_PATH_HEADER).unwrap(),
            "%2FGoogleDrive%2FResource%20Uploads%2Ffile%20name.zip"
        );
        assert_eq!(
            request.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/octet-stream"
        );
        assert_eq!(request.headers().get(header::CONTENT_LENGTH).unwrap(), "3");
        assert_eq!(request.headers().get(OVERWRITE_HEADER).unwrap(), "false");
        assert_eq!(request.timeout(), Some(&OPENLIST_UPLOAD_TIMEOUT));
        assert!(!request
            .headers()
            .get(header::AUTHORIZATION)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("Bearer "));
        assert_eq!(
            request.body().and_then(|body| body.as_bytes()),
            Some(&b"abc"[..])
        );
    }

    #[test]
    fn build_put_request_rejects_blank_required_config() {
        let client = Client::new();
        let marker = "openlist:/GoogleDrive/uploads/resource-9-123.zip";
        let missing_base_url = openlist_config("", "token", "openlist:/GoogleDrive/uploads");
        let missing_token = openlist_config(
            "https://openlist.example",
            "",
            "openlist:/GoogleDrive/uploads",
        );

        assert_eq!(
            build_put_request(&client, &missing_base_url, marker, b"abc")
                .expect_err("blank base url should fail")
                .code,
            50201
        );
        assert_eq!(
            build_put_request(&client, &missing_token, marker, b"abc")
                .expect_err("blank token should fail")
                .code,
            50201
        );
    }

    #[test]
    fn openlist_response_success_requires_http_success_and_code_200() {
        assert!(openlist_upload_succeeded(
            StatusCode::OK,
            &OpenListPutResponse { code: 200 }
        ));
        assert!(!openlist_upload_succeeded(
            StatusCode::OK,
            &OpenListPutResponse { code: 500 }
        ));
        assert!(!openlist_upload_succeeded(
            StatusCode::INTERNAL_SERVER_ERROR,
            &OpenListPutResponse { code: 200 }
        ));
    }

    #[tokio::test]
    async fn upload_resource_bytes_sends_contract_to_openlist() {
        let captured = Arc::new(Mutex::new(None));
        let app = Router::new()
            .route("/api/fs/put", put(capture_openlist_put))
            .with_state(Arc::clone(&captured));
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("test listener should bind");
        let address = listener
            .local_addr()
            .expect("test listener address should be readable");
        let server = tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("test OpenList server should run");
        });
        let client = Client::new();
        let config = openlist_config(
            &format!("http://{address}"),
            "upload-token",
            "openlist:/GoogleDrive/uploads",
        );

        upload_resource_bytes(
            &client,
            &config,
            "openlist:/GoogleDrive/uploads/resource-9-123.zip",
            b"archive-bytes",
        )
        .await
        .expect("OpenList upload should succeed");
        server.abort();

        let captured = captured
            .lock()
            .expect("captured request lock should not be poisoned")
            .clone()
            .expect("request should be captured");
        assert_eq!(captured.authorization.as_deref(), Some("upload-token"));
        assert_eq!(
            captured.file_path.as_deref(),
            Some("%2FGoogleDrive%2Fuploads%2Fresource-9-123.zip")
        );
        assert_eq!(
            captured.content_type.as_deref(),
            Some("application/octet-stream")
        );
        assert_eq!(captured.content_length.as_deref(), Some("13"));
        assert_eq!(captured.overwrite.as_deref(), Some("false"));
        assert_eq!(captured.body, b"archive-bytes");
    }

    async fn capture_openlist_put(
        State(captured): State<Arc<Mutex<Option<CapturedRequest>>>>,
        headers: HeaderMap,
        body: Bytes,
    ) -> Json<Value> {
        let request = CapturedRequest {
            authorization: header_value(&headers, header::AUTHORIZATION.as_str()),
            file_path: header_value(&headers, FILE_PATH_HEADER),
            content_type: header_value(&headers, header::CONTENT_TYPE.as_str()),
            content_length: header_value(&headers, header::CONTENT_LENGTH.as_str()),
            overwrite: header_value(&headers, OVERWRITE_HEADER),
            body: body.to_vec(),
        };
        *captured
            .lock()
            .expect("captured request lock should not be poisoned") = Some(request);

        Json(json!({ "code": 200 }))
    }

    fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string)
    }

    fn openlist_config(base_url: &str, token: &str, resource_upload_dir: &str) -> OpenListConfig {
        OpenListConfig {
            base_url: base_url.to_string(),
            token: token.to_string(),
            resource_upload_dir: resource_upload_dir.to_string(),
        }
    }
}
