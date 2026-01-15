use crate::domain::user::{
    entity::UserId,
    repository::UserRepository,
};

pub struct InactiveUser<'a> {
    repo: &'a dyn UserRepository,
}

impl<'a> InactiveUser<'a> {
    pub fn new(repo: &'a dyn UserRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, user_id: UserId) -> Result<(), String> {
        // 1️⃣ ambil user
        let mut user = self
            .repo
            .find_by_id(&user_id)
            .await
            .map_err(|e| format!("database error: {}", e))?
            .ok_or("user not found".to_string())?;

        // 2️⃣ business logic
        user.deactivate();

        // 3️⃣ simpan perubahan
        self.repo
            .save(&user)
            .await
            .map_err(|e| format!("failed to save user: {}", e))?;

        Ok(())
    }
}
