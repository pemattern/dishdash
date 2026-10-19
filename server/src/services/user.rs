use uuid::Uuid;

use crate::{
    error::ServiceResult,
    models::{BaseModel, User},
    repositories::{Repository, SelectById},
    schemas::user::{PatchUserRequest, PostUserRequest},
};

#[derive(Clone)]
pub struct UserService<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> ServiceResult<Option<BaseModel<User>>> {
        Ok(User::select_by_id(self.repository, id).await?)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<User>> {
        Ok(self.user_repository.find_by_email(email).await?)
    }

    pub async fn create_user(&self, user: PostUserRequest) -> ServiceResult<User> {
        // TODO: Hash Password
        let password_hash = user.password;
        Ok(self
            .user_repository
            .create(user.email, password_hash)
            .await?)
    }

    pub async fn update_user(&self, id: Uuid, user: PatchUserRequest) -> ServiceResult<User> {
        Ok(self.user_repository.update(id, user.email).await?)
    }

    pub async fn delete_user(&self, id: Uuid) -> ServiceResult<User> {
        Ok(self.user_repository.delete(id).await?)
    }
}
