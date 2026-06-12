use sqlx::PgPool;

use crate::models::user::UserRow;

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, LOWER($2), $3)
        RETURNING id, username, email, password_hash, avatar_url, role, created_at, updated_at
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, avatar_url, role, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, avatar_url, role, created_at, updated_at
        FROM users
        WHERE email = LOWER($1)
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, username, email, password_hash, avatar_url, role, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

pub async fn update_user_profile(
    pool: &PgPool,
    id: i64,
    username: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        UPDATE users
        SET username = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, username, email, password_hash, avatar_url, role, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(username)
    .fetch_one(pool)
    .await
}

pub async fn update_user_password_hash(
    pool: &PgPool,
    id: i64,
    password_hash: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        UPDATE users
        SET password_hash = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, username, email, password_hash, avatar_url, role, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn update_user_avatar_url(
    pool: &PgPool,
    id: i64,
    avatar_url: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        UPDATE users
        SET avatar_url = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING id, username, email, password_hash, avatar_url, role, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
}
