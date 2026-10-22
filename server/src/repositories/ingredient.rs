use crate::{error::RepositoryResult, models::ingredient::Ingredient};
use sqlx::PgExecutor;
use uuid::Uuid;

pub struct Ingredients;

impl Ingredients {
    pub async fn create<'a>(
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
}
