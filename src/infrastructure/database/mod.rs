use sqlx::{AnyPool, any::AnyPoolOptions};
use tracing::info;

pub async fn connect(database_url: &str) -> AnyPool {
    info!("Connecting to database...");
    AnyPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("failed to connect database")
}
