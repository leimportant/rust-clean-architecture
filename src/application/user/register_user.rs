use crate::domain::user::{
    entity::User,
    repository::UserRepository,
};
use crate::domain::security::password::PasswordService;
use uuid::Uuid;

pub struct RegisterUser<'a> {
    repo: &'a dyn UserRepository,
}

impl<'a> RegisterUser<'a> {
    pub fn new(repo: &'a dyn UserRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, email: String, username: String, name: String, password: String) -> Result<(), String> {
        if self.repo.find_by_email(&email).await.is_err() {
            return Err("email already registered".into());
        }

        let new_password = PasswordService::hash(&password)?;

        let user = User::new(
            Uuid::new_v4().to_string(),
            email,
            username,
            name,
            new_password,
            "Y".to_string(),
        );

        self.repo.save(&user).await;
        Ok(())
    }
}
