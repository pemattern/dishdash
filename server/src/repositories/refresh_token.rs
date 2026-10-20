use crate::{
    error::RepositoryResult,
    models::{BaseModel, refresh_token::RefreshToken},
    repositories::PostgresRepository,
};
use uuid::Uuid;

impl PostgresRepository<RefreshToken> {
    pub async fn mark_as_used(&self, id: Uuid) -> RepositoryResult<BaseModel<RefreshToken>> {
        let query = "
            UPDATE refresh_tokens
            SET is_used = true
            WHERE id = $1
            RETURNING *;";
        let refresh_token = sqlx::query_as::<_, BaseModel<RefreshToken>>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(refresh_token)
    }

    pub async fn insert(
        &self,
        refresh_token: RefreshToken,
    ) -> RepositoryResult<BaseModel<RefreshToken>> {
        let query = "
            INSERT INTO refresh_tokens (user_id, parent_id, opaque_token_hash)
            VALUES ($1, $2, $3)
            RETURNING *;";
        let refresh_token = sqlx::query_as::<_, BaseModel<RefreshToken>>(query)
            .bind(refresh_token.user_id)
            .bind(refresh_token.parent_id)
            .bind(refresh_token.opaque_token_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(refresh_token)
    }
}
