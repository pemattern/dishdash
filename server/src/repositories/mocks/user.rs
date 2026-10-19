use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::{BaseModel, User},
    repositories::{mocks::MockRepository, user::UserRepository},
};

#[async_trait]
impl UserRepository for MockRepository<User> {
    async fn insert(
        &self,
        email: String,
        password_hash: String,
    ) -> RepositoryResult<BaseModel<User>> {
        todo!()
    }
    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<BaseModel<User>> {
        todo!()
    }
    async fn select_by_email(&self, email: &str) -> RepositoryResult<Option<BaseModel<User>>> {
        todo!()
    }
}
