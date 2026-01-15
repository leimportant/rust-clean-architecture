use async_trait::async_trait;
use sqlx::AnyPool;

use crate::domain::user::{
    entity::{User, UserId},
    repository::UserRepository,
};
use crate::adapters::outbound::persistence::sql::user_queries as user_queries;

#[derive(Clone)]
pub struct UserRepositoryAdapter {
    pool: AnyPool,
}

impl UserRepositoryAdapter {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryAdapter {
    async fn save(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(user_queries::CREATE_USER_SQL)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.name)
            .bind(&user.password)
            .bind(&user.is_active)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(user_queries::FIND_USER_BY_EMAIL_SQL)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(user_queries::FIND_USER_BY_ID_SQL)
            .bind(&id.0)
            .fetch_optional(&self.pool)
            .await
    }
}
