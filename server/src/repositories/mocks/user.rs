use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::{BaseModel, User},
    repositories::{
        mocks::{MockRepository, assert_exactly_one, assert_one_or_none},
        user::UserRepository,
    },
};

#[async_trait]
impl UserRepository for MockRepository<User> {
    async fn insert(
        &self,
        email: String,
        password_hash: String,
    ) -> RepositoryResult<BaseModel<User>> {
        let user = User {
            email,
            password_hash,
        };
        let model = BaseModel::new(user);
        let mut lock = self.rows.lock().unwrap();
        lock.push(model.clone());
        Ok(model)
    }
    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<BaseModel<User>> {
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
