#[cfg(feature = "ssr")]
use {
    aws_config::BehaviorVersion,
    aws_sdk_s3::{Client, config::Credentials},
    axum::Router,
    cryptos_site::AppState,
    cryptos_site::app::*,
    cryptos_site::db::{create_pool, run_migrations},
    leptos::logging::log,
    leptos::prelude::*,
    leptos_axum::{LeptosRoutes, generate_route_list},
    std::env::var,
    tower_sessions::{Expiry, SessionManagerLayer},
    tower_sessions_sqlx_store::PostgresStore,
};

#[cfg(feature = "ssr")]
async fn create_s3_client() -> Client {
    let username = var("S3_USERNAME").expect("S3_USERNAME must be set in .env file");
    let password = var("S3_PASSWORD").expect("S3_PASSWORD must be set in .env file");
    let region = var("S3_REGION").expect("S3_REGION must be set in .env file");
    let endpoint_url = var("S3_ENDPOINT_URL").expect("S3_ENDPOINT_URL must be set in .env file");
    let creds = Credentials::new(username, password, None, None, "static");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(creds)
        .region(aws_config::Region::new(region))
        .endpoint_url(endpoint_url)
        .load()
        .await;
    Client::new(&config)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

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
        ));

    // Create app state
    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pool: pool.clone(),
        s3_client: create_s3_client().await,
    };

    let app = Router::new()
        .leptos_routes(&app_state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
