pub mod app;
pub mod auth;
pub mod models;

#[cfg(feature = "ssr")]
#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: leptos::config::LeptosOptions,
    pub pool: sqlx::PgPool,
    pub s3_client: aws_sdk_s3::Client,
}

#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
