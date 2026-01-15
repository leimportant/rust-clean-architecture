use sqlx::AnyPool;
use tracing::info;

pub async fn connect(db_url: &str) -> AnyPool {
    info!("connecting to database");
    let pool = AnyPool::connect(db_url)
        .await
        .expect("failed to connect database");
    info!("database connected");
    pool
}
