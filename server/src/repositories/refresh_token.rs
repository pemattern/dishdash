use crate::{
    error::RepositoryResult, models::refresh_token::RefreshToken, repositories::PostgresRepository,
};
use uuid::Uuid;

impl PostgresRepository<RefreshToken> {
    pub async fn mark_as_used(&self, id: Uuid) -> RepositoryResult<RefreshToken> {
        let query = "
            UPDATE refresh_tokens
            SET is_used = true
            WHERE id = $1
            RETURNING *;";
        let refresh_token = sqlx::query_as::<_, RefreshToken>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(refresh_token)
    }

    pub async fn insert(
        &self,
        user_id: Uuid,
        parent_id: Option<Uuid>,
        opaque_token_hash: String,
    ) -> RepositoryResult<RefreshToken> {
        let query = "
            INSERT INTO refresh_tokens (user_id, parent_id, opaque_token_hash)
            VALUES ($1, $2, $3)
            RETURNING *;";
        let refresh_token = sqlx::query_as::<_, RefreshToken>(query)
            .bind(user_id)
            .bind(parent_id)
            .bind(opaque_token_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(refresh_token)
    }
}
