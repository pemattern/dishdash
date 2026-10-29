use crate::{error::RepositoryResult, models::ingredient::Ingredient};
use sqlx::PgExecutor;
use uuid::Uuid;

pub struct Ingredients;

impl Ingredients {
    pub async fn select_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<Option<Ingredient>> {
        let user = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients WHERE id = $1;", id)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        name: String,
    ) -> RepositoryResult<Ingredient> {
        let ingredient = sqlx::query_as!(
            Ingredient,
            "INSERT INTO ingredients (name)
            VALUES ($1)
            RETURNING *;",
            name,
        )
        .fetch_one(executor)
        .await?;
        Ok(ingredient)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
        name: Option<String>,
    ) -> RepositoryResult<Ingredient> {
        let ingredient = sqlx::query_as!(
            Ingredient,
            "UPDATE ingredients
            SET
                name = COALESCE($1, name)
            WHERE id = $2
            RETURNING *;",
            name,
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(ingredient)
    }

    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<Ingredient> {
        let user = sqlx::query_as!(
            Ingredient,
            "DELETE FROM ingredients WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_one(executor)
        .await?;
        Ok(user)
    }
}
