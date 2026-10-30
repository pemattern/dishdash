use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::RepositoryResult, models::user::User};

pub struct Ingredients;

impl Ingredients {
    pub async fn select_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<Option<User>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1;", id)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn select_by_email<'a>(
        executor: impl PgExecutor<'a>,
        email: &str,
    ) -> RepositoryResult<Option<User>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        email: &str,
        password_hash: &str,
    ) -> RepositoryResult<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING *;",
            email,
            password_hash,
        )
        .fetch_one(executor)
        .await?;
        Ok(user)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
        email: Option<String>,
        password_hash: Option<String>,
    ) -> RepositoryResult<User> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users
            SET
                email = COALESCE($1, email),
                password_hash = COALESCE($2, password_hash)
            WHERE id = $3
            RETURNING *;",
            email,
            password_hash,
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(user)
    }

    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<User> {
        let user = sqlx::query_as!(User, "DELETE FROM users WHERE id = $1 RETURNING *;", id)
            .fetch_one(executor)
            .await?;
        Ok(user)
    }
}
