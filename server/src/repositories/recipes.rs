use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::recipe::{
        Recipe,
        schemas::{PatchRecipeSchema, PostRecipeSchema},
    },
};

pub struct Recipes;

impl Recipes {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<Option<Recipe>> {
        let user = sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE id = $1;", id)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> RepositoryResult<Vec<Recipe>> {
        let user = sqlx::query_as!(Recipe, "SELECT * FROM recipes;")
            .fetch_all(executor)
            .await?;
        Ok(user)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe: PostRecipeSchema,
    ) -> RepositoryResult<Recipe> {
        let recipe = sqlx::query_as!(
            Recipe,
            "INSERT INTO recipes (name, summary)
            VALUES ($1, $2)
            RETURNING *;",
            recipe.name,
            recipe.summary,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        recipe: PatchRecipeSchema,
    ) -> RepositoryResult<Recipe> {
        let recipe = sqlx::query_as!(
            Recipe,
            "UPDATE recipes
            SET
                name = COALESCE($1, name),
                summary = COALESCE($2, summary)
            WHERE id = $3
            RETURNING *;",
            recipe.name,
            recipe.summary,
            recipe_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }
    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> RepositoryResult<Recipe> {
        let user = sqlx::query_as!(Recipe, "DELETE FROM recipes WHERE id = $1 RETURNING *;", id)
            .fetch_one(executor)
            .await?;
        Ok(user)
    }
}
