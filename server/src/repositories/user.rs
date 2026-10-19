use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::{BaseModel, User},
    repositories::{PostgresRepository, Repository},
};

#[async_trait]
pub trait UserRepository: Repository<User> {
    async fn insert(
        &self,
        email: String,
        password_hash: String,
    ) -> RepositoryResult<BaseModel<User>>;
    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<BaseModel<User>>;
    async fn select_by_email(&self, email: &str) -> RepositoryResult<Option<BaseModel<User>>>;
}

#[async_trait]
impl UserRepository for PostgresRepository<User> {
    async fn insert(
        &self,
        email: String,
        password_hash: String,
    ) -> RepositoryResult<BaseModel<User>> {
        let query = "
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING *;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(email)
            .bind(password_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<BaseModel<User>> {
        let query = "
            UPDATE users
            SET
                email = COALESCE($1, email)
            WHERE
                id = $2
            RETURNING *;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(id)
            .bind(email)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn select_by_email(&self, email: &str) -> RepositoryResult<Option<BaseModel<User>>> {
        let query = "
            SELECT * FROM users
            WHERE email = $1;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }
}
