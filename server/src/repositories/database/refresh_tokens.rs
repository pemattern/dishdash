use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::AppResult, models::refresh_token::RefreshTokenModel};

pub struct RefreshTokens;

impl RefreshTokens {
    pub async fn select_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> AppResult<RefreshTokenModel> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            "SELECT * FROM refresh_tokens
            WHERE id = $1;",
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(refresh_token)
    }

    pub async fn mark_as_used<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> AppResult<RefreshTokenModel> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            "UPDATE refresh_tokens
            SET is_used = true
            WHERE id = $1
            RETURNING *;",
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(refresh_token)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        user_id: Uuid,
        parent_id: Option<Uuid>,
        opaque_token_hash: String,
    ) -> AppResult<RefreshTokenModel> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            "INSERT INTO refresh_tokens (user_id, parent_id, opaque_token_hash)
            VALUES ($1, $2, $3)
            RETURNING *;",
            user_id,
            parent_id,
            opaque_token_hash,
        )
        .fetch_one(executor)
        .await?;
        Ok(refresh_token)
    }
}
