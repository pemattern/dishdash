use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::{BaseModel, user::User},
    repositories::{
        Update,
        mocks::{MockRepository, assert_exactly_one, assert_one_or_none},
        user::UserRepository,
    },
    schemas::user::PatchUserRequest,
};

#[async_trait]
impl UserRepository for MockRepository<User> {
    async fn select_by_email(&self, email: &str) -> RepositoryResult<Option<BaseModel<User>>> {
        let lock = self.rows.lock().unwrap();
        let users: Vec<BaseModel<User>> = lock
            .iter()
            .filter(|row| row.model.email == email)
            .map(|row| row.clone())
            .collect();
        let user = assert_one_or_none(users)?;
        Ok(user)
    }
}

#[async_trait]
impl Update<User> for MockRepository<User> {
    async fn update(&self, id: Uuid, user: PatchUserRequest) -> RepositoryResult<BaseModel<User>> {
        let email = user.email;
        let lock = self.rows.lock().unwrap();
        let users: Vec<BaseModel<User>> = lock
            .iter()
            .filter(|row| row.id == id)
            .map(|row| row.clone())
            .collect();
        let mut user = assert_exactly_one(users)?;
        let Some(email) = email else {
            return Ok(user);
        };
        user.model.email = email;
        Ok(user)
    }
}
