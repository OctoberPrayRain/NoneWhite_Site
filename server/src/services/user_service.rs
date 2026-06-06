use sqlx::PgPool;

use crate::{
    dto::users::{ChangePasswordRequest, UpdateUserProfileRequest, UserResponse},
    error::{AppError, AppResult},
    repositories::user_repository,
    services::auth_service,
};

pub async fn get_current_user(pool: &PgPool, user_id: i64) -> AppResult<UserResponse> {
    let user = user_repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::user_not_found)?;

    Ok(UserResponse::from(user))
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: i64,
    request: UpdateUserProfileRequest,
) -> AppResult<UserResponse> {
    if request.avatar_url.is_some() {
        return Err(AppError::avatar_url_update_forbidden());
    }

    let username = request
        .username
        .as_deref()
        .ok_or_else(AppError::username_invalid)
        .and_then(auth_service::validate_username)?;

    let user = user_repository::update_user_profile(pool, user_id, username)
        .await
        .map_err(map_update_profile_error)?;

    Ok(UserResponse::from(user))
}

pub async fn change_password(
    pool: &PgPool,
    user_id: i64,
    request: ChangePasswordRequest,
) -> AppResult<()> {
    if request.current_password.is_empty() {
        return Err(AppError::password_required());
    }
    auth_service::validate_registration_password(&request.new_password)?;

    let user = user_repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|_| AppError::internal())?
        .ok_or_else(AppError::user_not_found)?;

    let current_password_matches =
        auth_service::verify_password(request.current_password, user.password_hash).await?;
    if !current_password_matches {
        return Err(AppError::current_password_incorrect());
    }

    let new_password_hash = auth_service::hash_password(request.new_password).await?;
    user_repository::update_user_password_hash(pool, user_id, &new_password_hash)
        .await
        .map_err(|_| AppError::internal())?;

    Ok(())
}

fn map_update_profile_error(error: sqlx::Error) -> AppError {
    match error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23505") =>
        {
            AppError::username_taken()
        }
        _ => AppError::internal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn update_profile_rejects_avatar_url() {
        let pool = PgPool::connect_lazy("postgres://user:password@localhost/db")
            .expect("lazy pool should be created");
        let error = update_profile(
            &pool,
            1,
            UpdateUserProfileRequest {
                username: Some("alice".to_string()),
                avatar_url: Some("/avatar.png".to_string()),
            },
        )
        .await
        .expect_err("avatarUrl should be rejected before DB access");

        assert_eq!(error.code, 40005);
    }
}
