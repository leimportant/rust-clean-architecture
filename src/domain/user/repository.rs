use async_trait::async_trait;
use crate::domain::user::entity::{User, UserId};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, sqlx::Error>;
}
