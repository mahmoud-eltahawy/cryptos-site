#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use super::models::Estate;

#[cfg(feature = "ssr")]
pub async fn create_estate(
    pool: &PgPool,
    name: String,
    address: String,
    image_url: String,
    price_in_cents: i64,
    space_in_meters: i32,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
            INSERT INTO estates (name, address, image_url, price_in_cents, space_in_meters)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        &name,
        &address,
        &image_url,
        price_in_cents,
        space_in_meters
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_estate_by_id(pool: &PgPool, id: Uuid) -> Result<Estate, Error> {
    let estate = sqlx::query_as!(
        Estate,
        r#"
        SELECT id, name, address, image_url,description, price_in_cents, space_in_meters
        FROM estates
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(estate)
}

#[cfg(feature = "ssr")]
pub async fn get_all_estates(pool: &PgPool) -> Result<Vec<Estate>, Error> {
    let estates = sqlx::query_as!(
        Estate,
        r#"
        SELECT id, name, address, image_url,description, price_in_cents, space_in_meters
        FROM estates
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(estates)
}

#[cfg(feature = "ssr")]
pub async fn update_estate_name(pool: &PgPool, id: Uuid, name: String) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET name = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        &name,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update_estate_address(pool: &PgPool, id: Uuid, address: String) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET address = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        &address,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update_estate_image_url(
    pool: &PgPool,
    id: Uuid,
    image_url: String,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET image_url = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        &image_url,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update_description(pool: &PgPool, id: Uuid, description: String) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET description = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        &description,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update_estate_price(
    pool: &PgPool,
    id: Uuid,
    price_in_cents: i64,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET price_in_cents = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        price_in_cents,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn update_estate_space(
    pool: &PgPool,
    id: Uuid,
    space_in_meters: i32,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE estates
        SET space_in_meters = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        space_in_meters,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn delete_estate(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM estates
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn count_estates(pool: &PgPool) -> Result<i64, Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM estates
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(count.unwrap_or(0))
}
