use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::get_hash,
    error::ServiceResult,
    models::user::User,
    repositories::users::Ingredients,
    schemas::user::{PatchUserRequest, PostUserRequest},
};

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: &PgPool) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> ServiceResult<Option<User>> {
        let user = Ingredients::select_by_id(&self.pool, id).await?;
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<User>> {
        let user = Ingredients::select_by_email(&self.pool, email).await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: PostUserRequest) -> ServiceResult<User> {
        let password_hash = get_hash(&user.password)?;
        let user = Ingredients::insert(&self.pool, &user.email, &password_hash).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, user: PatchUserRequest) -> ServiceResult<User> {
        let user = Ingredients::update(&self.pool, id, user).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> ServiceResult<User> {
        let user = Ingredients::delete_by_id(&self.pool, id).await?;
        Ok(user)
    }
}
