use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::RepositoryResult, models::recipe::Recipe};

pub struct Recipes;

impl Recipes {
    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        name: String,
        summary: String,
    ) -> RepositoryResult<Recipe> {
        let recipe = sqlx::query_as!(
            Recipe,
            "INSERT INTO recipes (name, summary)
            VALUES ($1, $2)
            RETURNING *;",
            name,
            summary,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<Recipe> {
        let recipe = sqlx::query_as!(
            Recipe,
            "UPDATE recipes
            SET
                name = COALESCE($1, name),
                summary = COALESCE($2, summary)
            WHERE id = $3
            RETURNING *;",
            name,
            summary,
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }
}
