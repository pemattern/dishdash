use crate::{
    auth::password::get_hash,
    error::ServiceResult,
    models::{BaseModel, user::User},
    repositories::PostgresRepository,
    schemas::user::{PatchUserRequest, PostUserRequest},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repository: PostgresRepository<User>,
}

impl UserService {
    pub fn new(pool: &PgPool) -> Self {
        let user_repository = PostgresRepository::<User>::new(pool.clone());
        Self { user_repository }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> ServiceResult<Option<BaseModel<User>>> {
        Ok(self.user_repository.select_by_id(id).await?)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<BaseModel<User>>> {
        Ok(self.user_repository.select_by_email(email).await?)
    }

    pub async fn create_user(&self, user: PostUserRequest) -> ServiceResult<BaseModel<User>> {
        let password_hash = get_hash(&user.password)?;
        let user = User::new(user.email, password_hash);
        Ok(self.user_repository.insert(user).await?)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        user: PatchUserRequest,
    ) -> ServiceResult<BaseModel<User>> {
        Ok(self.user_repository.update(id, user).await?)
    }

    pub async fn delete_user(&self, id: Uuid) -> ServiceResult<BaseModel<User>> {
        Ok(self.user_repository.delete_by_id(id).await?)
    }
}
