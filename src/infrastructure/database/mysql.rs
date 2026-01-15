#[derive(Clone)]
pub struct MySqlPool {
    pub url: String,
}

pub fn connect(database_url: &str) -> MySqlPool {
    MySqlPool {
        url: database_url.to_string(),
    }
}
