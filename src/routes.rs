use tower_http::trace::TraceLayer;
use axum::{Router, routing::get};
use crate::app::App;
use crate::adapters::inbound::http::user;

pub fn routes(app: App) -> Router {
    Router::new()
        .route("/", get(|| async { "OK 🚀 API is running" }))
        .nest("/api", user::routes(app))
        .layer(TraceLayer::new_for_http())
}
