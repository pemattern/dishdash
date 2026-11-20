use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::password::get_hash,
    error::AppResult,
    models::user::{PatchUserSchema, PostUserSchema, User},
    repositories::users::Users,
    requests::patch_field::PatchField,
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
        let user = Users::select_by_id(&self.pool, id).await?;
        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> AppResult<User> {
        let user = Users::select_by_email(&self.pool, email).await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: PostUserSchema) -> AppResult<User> {
        let password_hash = get_hash(&user.password)?;
        let user = Users::insert(&self.pool, &user.email, &password_hash).await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, mut user: PatchUserSchema) -> AppResult<User> {
        if let PatchField::Value(password) = user.password {
            user.password = PatchField::Value(get_hash(&password)?);
        }
        let user_updates = user.try_into()?;
        let user = Users::update(&self.pool, id, user_updates).await?;
        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<User> {
        let user = Users::delete_by_id(&self.pool, id).await?;
        Ok(user)
    }
}
