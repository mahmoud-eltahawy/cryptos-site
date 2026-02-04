#[cfg(feature = "ssr")]
use {
    sqlx::{Error, PgPool},
    uuid::Uuid,
};

#[cfg(feature = "ssr")]
use super::models::User;
#[cfg(feature = "ssr")]
use crate::auth::Level;

#[cfg(feature = "ssr")]
pub async fn create_user(
    pool: &PgPool,
    name: String,
    password: String,
    level: Level,
) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, password, level)
        VALUES ($1, $2, $3)
        RETURNING id, name, password, level, created_at, updated_at
        "#,
    )
    .bind(&name)
    .bind(&password)
    .bind(&level)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, password, level, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn get_user_by_name(pool: &PgPool, name: &str) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, password, level, created_at, updated_at
        FROM users
        WHERE name = $1
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, password, level, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

#[cfg(feature = "ssr")]
pub async fn update_user_name(pool: &PgPool, id: Uuid, name: String) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, name, password, level, created_at, updated_at
        "#,
    )
    .bind(&name)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn update_user_password(
    pool: &PgPool,
    id: Uuid,
    password: String,
) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET password = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, name, password, level, created_at, updated_at
        "#,
    )
    .bind(&password)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn update_user_level(pool: &PgPool, id: Uuid, level: Level) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET level = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, name, password, level, created_at, updated_at
        "#,
    )
    .bind(&level)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn count_users(pool: &PgPool) -> Result<i64, Error> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM users
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(count)
}
