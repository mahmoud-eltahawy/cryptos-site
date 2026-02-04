#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use cryptos_site::app::*;
    use cryptos_site::db::{create_pool, run_migrations};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_sqlx_store::PostgresStore;

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create database connection pool
    log!("Connecting to database...");
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    log!("Running database migrations...");
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    log!("Database setup complete!");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Set up session store with PostgreSQL
    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to migrate session store");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::seconds(3600),
        )); // 1 hour

    // Create app state
    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pool: pool.clone(),
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(session_layer)
        .with_state(app_state);

    // run our app with hyper
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: leptos::config::LeptosOptions,
    pub pool: sqlx::PgPool,
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
