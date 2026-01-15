use crate::domain::user::{
    entity::UserId,
    repository::UserRepository,
};

pub struct ChangePassword<'a> {
    repo: &'a dyn UserRepository,
}

impl<'a> ChangePassword<'a> {
    pub fn new(repo: &'a dyn UserRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        user_id: UserId,
        new_password: String,
    ) -> Result<(), String> {
        let mut user = self.repo
            .find_by_id(&user_id)
            .await
            .map_err(|e| format!("database error: {}", e))?
            .ok_or("user not found".to_string())?;


        user.change_password(new_password);

        self.repo.save(&user).await;

        Ok(())
    }
}
