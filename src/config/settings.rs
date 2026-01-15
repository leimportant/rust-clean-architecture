#[derive(Clone)]
pub enum DbDriver {
    MySql,
    Postgres,
}

#[derive(Clone)]
pub struct Settings {
    pub db_driver: DbDriver,
    pub db_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        let driver = std::env::var("DB_DRIVER")
            .unwrap_or("mysql".into());

        let db_driver = match driver.as_str() {
            "postgres" => DbDriver::Postgres,
            _ => DbDriver::MySql,
        };

        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self { db_driver, db_url }
    }
}


