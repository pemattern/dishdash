use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::get_hash,
    error::AppResult,
    payloads::user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    repositories::database::Users,
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

    pub async fn find_user_by_id(&self, id: Uuid) -> AppResult<UserResponse> {
        let user = Users::select_by_id(&self.pool, id).await?;
        Ok(user.into())
    }

    pub async fn find_user_by_email(&self, email: &str) -> AppResult<UserResponse> {
        let user = Users::select_by_email(&self.pool, email).await?;
        Ok(user.into())
    }

    pub async fn create_user(&self, user: CreateUserRequest) -> AppResult<UserResponse> {
        let password_hash = get_hash(&user.password)?;
        let user = Users::insert(&self.pool, &user.email, &password_hash).await?;
        Ok(user.into())
    }

    pub async fn update_user(&self, id: Uuid, user: UpdateUserRequest) -> AppResult<UserResponse> {
        let mut pasword_hash = None;
        if let Some(password) = &user.password {
            pasword_hash = Some(get_hash(&password)?);
        }
        let user = Users::update(&self.pool, id, user, pasword_hash).await?;
        Ok(user.into())
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<UserResponse> {
        let user = Users::delete(&self.pool, id).await?;
        Ok(user.into())
    }
}
