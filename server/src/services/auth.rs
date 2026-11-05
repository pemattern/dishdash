use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use crate::{
    auth::{
        password::{get_hash, is_valid_password},
        token::generate_refresh_token,
    },
    error::{AppError, AppResult},
    models::{refresh_token::RefreshToken, user::User},
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
        if let user = Ingredients::select_by_email(&self.pool, email).await?
            && is_valid_password(password, &user.password_hash)?
        {
            return Ok(user);
        };
        Err(AppError::IncorrectPassword)
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
        if let Some((refresh_token_id_raw, opaque_token_raw)) =
            composite_refresh_token.split_once('.')
            && let Ok(refresh_token_id) = Uuid::parse_str(refresh_token_id_raw)
            && let refresh_token = RefreshTokens::select_by_id(&self.pool, refresh_token_id).await?
            && refresh_token.is_valid()
            && is_valid_password(opaque_token_raw, &refresh_token.opaque_token_hash)?
        {
            return Ok(refresh_token);
        };
        Err(AppError::InvalidRefreshToken)
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
