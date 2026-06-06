use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{error::DatabaseError, PgPool};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task;

use crate::{
    config::AuthConfig,
    dto::{
        auth::{AuthTokenResponse, LoginRequest, RegisterRequest},
        users::UserResponse,
    },
    error::{AppError, AppResult},
    models::user::UserRow,
    repositories::user_repository,
};

const MIN_USERNAME_LENGTH: usize = 3;
const MAX_USERNAME_LENGTH: usize = 32;
const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_BCRYPT_PASSWORD_BYTES: usize = 72;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn register_user(pool: &PgPool, request: RegisterRequest) -> AppResult<UserResponse> {
    let username = validate_username(&request.username)?;
    let email = validate_email(&request.email)?;
    validate_registration_password(&request.password)?;

    let password_hash = hash_password(request.password).await?;
    let user = user_repository::create_user(pool, username, &email, &password_hash)
        .await
        .map_err(map_create_user_error)?;

    Ok(UserResponse::from(user))
}

pub async fn login_user(
    pool: &PgPool,
    auth_config: &AuthConfig,
    request: LoginRequest,
) -> AppResult<AuthTokenResponse> {
    let email = validate_email(&request.email)?;
    if request.password.is_empty() {
        return Err(AppError::password_required());
    }

    let user = user_repository::find_user_by_email(pool, &email)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::invalid_credentials)?;

    let password_matches = verify_password(request.password, user.password_hash.clone()).await?;
    if !password_matches {
        return Err(AppError::invalid_credentials());
    }

    let token = issue_token(&user, auth_config)?;

    Ok(AuthTokenResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: auth_config.jwt_expires_in_seconds,
        user: UserResponse::from(user),
    })
}

pub(crate) fn validate_username(username: &str) -> AppResult<&str> {
    let trimmed = username.trim();
    if trimmed.len() < MIN_USERNAME_LENGTH || trimmed.len() > MAX_USERNAME_LENGTH {
        return Err(AppError::username_invalid());
    }

    Ok(trimmed)
}

fn validate_email(email: &str) -> AppResult<String> {
    let normalized = email.trim().to_ascii_lowercase();
    let has_single_at = normalized.matches('@').count() == 1;
    let has_domain_dot = normalized
        .split_once('@')
        .map(|(_, domain)| {
            domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
        })
        .unwrap_or(false);

    if normalized.is_empty() || !has_single_at || !has_domain_dot {
        return Err(AppError::email_invalid());
    }

    Ok(normalized)
}

pub(crate) fn validate_registration_password(password: &str) -> AppResult<()> {
    if password.len() < MIN_PASSWORD_LENGTH || password.as_bytes().len() > MAX_BCRYPT_PASSWORD_BYTES
    {
        return Err(AppError::password_too_short());
    }

    Ok(())
}

pub(crate) async fn hash_password(password: String) -> AppResult<String> {
    task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|_| AppError::internal())?
        .map_err(|_| AppError::internal())
}

pub(crate) async fn verify_password(password: String, password_hash: String) -> AppResult<bool> {
    task::spawn_blocking(move || verify(password, &password_hash))
        .await
        .map_err(|_| AppError::internal())?
        .map_err(|_| AppError::internal())
}

fn issue_token(user: &UserRow, auth_config: &AuthConfig) -> AppResult<String> {
    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::internal())?
        .as_secs() as usize;
    let expires_at = issued_at
        .checked_add(auth_config.jwt_expires_in_seconds as usize)
        .ok_or_else(AppError::internal)?;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expires_at,
        iat: issued_at,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::internal())
}

pub fn verify_token(token: &str, auth_config: &AuthConfig) -> AppResult<i64> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 60;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| AppError::invalid_token())?;

    token_data
        .claims
        .sub
        .parse::<i64>()
        .map_err(|_| AppError::invalid_token())
}

fn map_create_user_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::Database(database_error) => map_database_conflict(database_error.as_ref()),
        _ => AppError::internal(),
    }
}

fn map_database_conflict(error: &dyn DatabaseError) -> AppError {
    if error.code().as_deref() == Some("23505") {
        let constraint = error.constraint().unwrap_or_default();
        if constraint.contains("username") {
            return AppError::username_taken();
        }
        if constraint.contains("email") {
            return AppError::email_registered();
        }
        return AppError::new(StatusCode::CONFLICT, 40900, "Resource already exists");
    }

    AppError::internal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_email_rejects_invalid_email() {
        let error = validate_email("not-an-email").expect_err("email should be invalid");

        assert_eq!(error.code, 40002);
    }

    #[test]
    fn validate_registration_password_rejects_short_password() {
        let error = validate_registration_password("short").expect_err("password should be short");

        assert_eq!(error.code, 40003);
    }

    #[test]
    fn validate_username_rejects_short_username() {
        let error = validate_username("ab").expect_err("username should be invalid");

        assert_eq!(error.code, 40001);
    }

    #[test]
    fn issue_and_verify_token_returns_user_id() {
        let auth_config = AuthConfig {
            jwt_secret: "test-secret".to_string(),
            jwt_expires_in_seconds: 60,
        };
        let user = UserRow {
            id: 42,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            role: "user".to_string(),
            created_at: sqlx::types::chrono::Utc::now(),
            updated_at: sqlx::types::chrono::Utc::now(),
        };

        let token = issue_token(&user, &auth_config).expect("token should be issued");
        let user_id = verify_token(&token, &auth_config).expect("token should verify");

        assert_eq!(user_id, 42);
    }
}
