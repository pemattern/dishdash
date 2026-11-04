use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::get_hash,
    error::AppResult,
    models::user::{
        User,
        schemas::{PatchUserSchema, PostUserSchema},
    },
    repositories::users::Ingredients,
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

    pub async fn find_user_by_id(&self, id: Uuid) -> AppResult<User> {
        let user = Ingredients::select_by_id(&self.pool, id).await?;
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> AppResult<User> {
        let user = Ingredients::select_by_email(&self.pool, email).await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: PostUserSchema) -> AppResult<User> {
        let password_hash = get_hash(&user.password)?;
        let user = Ingredients::insert(&self.pool, &user.email, &password_hash).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, user: PatchUserSchema) -> AppResult<User> {
        let mut password_hash = None;
        if let Some(password) = user.password {
            password_hash = Some(get_hash(&password)?);
        }
        let user = Ingredients::update(&self.pool, id, user.email, password_hash).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<User> {
        let user = Ingredients::delete_by_id(&self.pool, id).await?;
        Ok(user)
    }
}
