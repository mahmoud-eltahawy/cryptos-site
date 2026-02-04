#[cfg(feature = "ssr")]
use sqlx::postgres::PgPoolOptions;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};
#[cfg(feature = "ssr")]
use std::time::Duration;

pub use crate::models;
pub mod users;
pub mod estates;

#[cfg(feature = "ssr")]
pub type DbPool = Pool<Postgres>;

#[cfg(feature = "ssr")]
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

#[cfg(feature = "ssr")]
pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
