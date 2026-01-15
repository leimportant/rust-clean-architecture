use tokio::net::TcpListener;
use dotenvy::dotenv;
use sqlx::any::install_default_drivers; // 👈 TAMBAH INI
use tracing_subscriber::fmt;
mod adapters;
mod app;
mod application;
mod config;
mod domain;
mod infrastructure;
mod routes;

use crate::app::App;


fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

use tower_cookies::CookieManagerLayer;



#[tokio::main]
async fn main() {
    dotenv().ok();
     init_tracing();

    // 🔥 WAJIB sebelum AnyPool dipakai
    install_default_drivers();

    // Build application state (DB, repos, etc)
    let app_state = App::new().await;

    // Register all routes
   let app = routes::routes(app_state)
    .layer(CookieManagerLayer::new());

    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{port}");

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("🚀 Server running at http://{addr}");

    axum::serve(listener, app)
        .await
        .unwrap();
}
