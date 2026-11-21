use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::get_hash,
    error::AppResult,
    models::UserModel,
    payloads::user::{CreateUserRequest, UpdateUserRequest},
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

    pub async fn find_user_by_id(&self, id: Uuid) -> AppResult<UserModel> {
        let user = Users::select_by_id(&self.pool, id).await?;
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> AppResult<UserModel> {
        let user = Users::select_by_email(&self.pool, email).await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: CreateUserRequest) -> AppResult<UserModel> {
        let password_hash = get_hash(&user.password)?;
        let user = Users::insert(&self.pool, &user.email, &password_hash).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, user: UpdateUserRequest) -> AppResult<UserModel> {
        let mut pasword_hash = None;
        if let Some(password) = &user.password {
            pasword_hash = Some(get_hash(&password)?);
        }
        let user = Users::update(&self.pool, id, user, pasword_hash).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<UserModel> {
        let user = Users::delete(&self.pool, id).await?;
        Ok(user)
    }
}
