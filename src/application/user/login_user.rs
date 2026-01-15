use crate::domain::user::{
    repository::UserRepository,
    service::UserDomainService,
};

pub struct LoginUser<'a> {
    repo: &'a dyn UserRepository,
}

impl<'a> LoginUser<'a> {
    pub fn new(repo: &'a dyn UserRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<(), String> {
        let user = self.repo
            .find_by_email(email)
            .await
            .map_err(|e| format!("database error: {}", e))?
            .ok_or("user not found".to_string())?;

        if !UserDomainService::can_login(&user) {
            return Err("user inactive".into());
        }

        Ok(())
    }
}
