use crate::{
    error::RepositoryResult,
    models::{BaseModel, user::User},
    repositories::PostgresRepository,
    schemas::user::PatchUserRequest,
};
use uuid::Uuid;

impl PostgresRepository<User> {
    pub async fn select_by_email(&self, email: &str) -> RepositoryResult<Option<BaseModel<User>>> {
        let query = "
            SELECT * FROM users
            WHERE email = $1;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn insert(&self, user: User) -> RepositoryResult<BaseModel<User>> {
        let query = "
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING *;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(user.email)
            .bind(user.password_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn update(
        &self,
        id: Uuid,
        user: PatchUserRequest,
    ) -> RepositoryResult<BaseModel<User>> {
        let query = "
            UPDATE users
            SET
                email = COALESCE($1, email)
            WHERE
                id = $2
            RETURNING *;";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(id)
            .bind(user.email)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
