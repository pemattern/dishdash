use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use crate::{
    auth::{
        password::{get_hash, verify_password},
        token::{generate_refresh_token, parse_refresh_token, validate_refresh_token_expiry},
    },
    error::{AppError, AppResult},
    models::{
        refresh_token::{self, RefreshToken},
        user::User,
    },
    repositories::{refresh_tokens::RefreshTokens, users::Ingredients},
};

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: &PgPool) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn get_user_from_basic_auth(&self, email: &str, password: &str) -> AppResult<User> {
        let user = Ingredients::select_by_email(&self.pool, email).await?;
        verify_password(password, &user.password_hash)?;
        Ok(user)
    }

    pub async fn generate_refresh_token_from_login(&self, user_id: Uuid) -> AppResult<String> {
        generate_and_insert_refresh_token(&self.pool, user_id, None).await
    }

    pub async fn cycle_refresh_token(&self, refresh_token: &RefreshToken) -> AppResult<String> {
        let mut tx = self.pool.begin().await?;
        let _ = RefreshTokens::mark_as_used(&mut *tx, refresh_token.id).await?;
        let composite_token = generate_and_insert_refresh_token(
            &mut *tx,
            refresh_token.user_id,
            Some(refresh_token.id),
        )
        .await?;
        tx.commit().await?;
        Ok(composite_token)
    }

    pub async fn mark_refresh_token_as_used(
        &self,
        refresh_token: &RefreshToken,
    ) -> AppResult<RefreshToken> {
        let refresh_token = RefreshTokens::mark_as_used(&self.pool, refresh_token.id).await?;
        Ok(refresh_token)
    }

    pub async fn get_valid_refresh_token(
        &self,
        composite_refresh_token: &str,
    ) -> AppResult<RefreshToken> {
        let (refresh_token_id, opaque_token) = parse_refresh_token(composite_refresh_token)?;
        let refresh_token = RefreshTokens::select_by_id(&self.pool, refresh_token_id).await?;
        validate_refresh_token_expiry(&refresh_token)?;
        verify_password(&opaque_token, &refresh_token.opaque_token_hash)?;
        Ok(refresh_token)
    }
}

async fn generate_and_insert_refresh_token<'a>(
    executor: impl PgExecutor<'a>,
    user_id: Uuid,
    parent_id: Option<Uuid>,
) -> AppResult<String> {
    let opaque_token_raw = generate_refresh_token();
    let opaque_token_hash = get_hash(&opaque_token_raw)?;
    let refresh_token =
        RefreshTokens::insert(executor, user_id, parent_id, opaque_token_hash).await?;
    let composite_refresh_token = format!("{}.{}", refresh_token.id, opaque_token_raw);
    Ok(composite_refresh_token)
}
