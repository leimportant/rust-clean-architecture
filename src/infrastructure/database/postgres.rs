#[derive(Clone)]
pub struct PostgresPool {
    pub url: String,
}

pub fn connect(database_url: &str) -> PostgresPool {
    println!("Connecting to PostgreSQL: {}", database_url);

    PostgresPool {
        url: database_url.to_string(),
    }
}
