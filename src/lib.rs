use leptos::prelude::*;

pub mod app;
pub mod auth;
pub mod models;

#[cfg(feature = "ssr")]
pub mod db;
#[cfg(feature = "ssr")]
pub mod s3;

#[cfg(feature = "ssr")]
#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: leptos::config::LeptosOptions,
    pub pool: sqlx::PgPool,
    pub s3: s3::S3,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}

#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="text-center py-12">
            <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
            <p class="mt-4 text-gray-600">"جاري التحميل..."</p>
        </div>
    }
}
