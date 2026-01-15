use crate::config::settings::Settings;
use crate::infrastructure::database;
use crate::adapters::outbound::persistence::user_repository::UserRepositoryAdapter;

#[derive(Clone)]
pub struct App {
    pub user_repo: UserRepositoryAdapter,
}

impl App {
    pub async fn new() -> Self {
        let settings = Settings::from_env();

        let pool = database::connect(&settings.db_url).await;

        let user_repo = UserRepositoryAdapter::new(pool);

        Self { user_repo }
    }
}
