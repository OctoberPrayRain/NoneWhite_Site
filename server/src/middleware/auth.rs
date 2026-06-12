use axum::http::HeaderMap;

use crate::{
    config::AuthConfig,
    error::{AppError, AppResult},
    models::user::UserRow,
    repositories::user_repository,
    services::auth_service,
};
use sqlx::PgPool;

const BEARER_PREFIX: &str = "Bearer ";

pub fn authenticated_user_id(headers: &HeaderMap, auth_config: &AuthConfig) -> AppResult<i64> {
    let token = bearer_token(headers)?;
    auth_service::verify_token(token, auth_config)
}

pub async fn admin_user(
    headers: &HeaderMap,
    auth_config: &AuthConfig,
    pool: &PgPool,
) -> AppResult<UserRow> {
    let user_id = authenticated_user_id(headers, auth_config)?;
    let user = user_repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::user_not_found)?;

    if user.role != "admin" {
        return Err(AppError::permission_denied());
    }

    Ok(user)
}

fn bearer_token(headers: &HeaderMap) -> AppResult<&str> {
    let authorization = headers
        .get("Authorization")
        .ok_or_else(AppError::authentication_required)?
        .to_str()
        .map_err(|_| AppError::invalid_token())?;

    authorization
        .strip_prefix(BEARER_PREFIX)
        .filter(|token| !token.trim().is_empty())
        .ok_or_else(AppError::authentication_required)
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    #[test]
    fn bearer_token_requires_authorization_header() {
        let headers = HeaderMap::new();
        let error = bearer_token(&headers).expect_err("missing header should fail");

        assert_eq!(error.code, 40102);
    }

    #[test]
    fn bearer_token_rejects_non_bearer_header() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Basic abc"));

        let error = bearer_token(&headers).expect_err("non-bearer header should fail");

        assert_eq!(error.code, 40102);
    }

    #[test]
    fn bearer_token_extracts_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer token-123"),
        );

        let token = bearer_token(&headers).expect("bearer token should parse");

        assert_eq!(token, "token-123");
    }

    #[test]
    fn non_admin_permission_denied_code_is_stable() {
        let error = AppError::permission_denied();

        assert_eq!(error.status, axum::http::StatusCode::FORBIDDEN);
        assert_eq!(error.code, 40301);
        assert_eq!(error.message, "Permission denied");
    }
}
