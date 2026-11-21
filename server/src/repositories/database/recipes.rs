use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe::{PatchRecipeSchema, PostRecipeSchema, RecipeModel},
};

pub struct Recipes;

impl Recipes {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> AppResult<RecipeModel> {
        let user = sqlx::query_as!(RecipeModel, "SELECT * FROM recipes WHERE id = $1;", id)
            .fetch_one(executor)
            .await?;
        Ok(user)
    }

    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<RecipeModel>> {
        let user = sqlx::query_as!(RecipeModel, "SELECT * FROM recipes;")
            .fetch_all(executor)
            .await?;
        Ok(user)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe: &PostRecipeSchema,
        image_url: Option<String>,
    ) -> AppResult<RecipeModel> {
        let recipe = sqlx::query_as!(
            RecipeModel,
            "INSERT INTO recipes (name, image_url, duration_minutes)
            VALUES ($1, $2, $3)
            RETURNING *;",
            recipe.name,
            image_url,
            recipe.duration_minutes,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        recipe: PatchRecipeSchema,
    ) -> AppResult<RecipeModel> {
        let recipe = sqlx::query_as!(
            RecipeModel,
            "UPDATE recipes
            SET
                name = COALESCE($1, name),
                duration_minutes = COALESCE($2, duration_minutes)
            WHERE id = $3
            RETURNING *;",
            recipe.name,
            recipe.duration_minutes,
            recipe_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe)
    }
    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
    ) -> AppResult<RecipeModel> {
        let user = sqlx::query_as!(
            RecipeModel,
            "DELETE FROM recipes WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_one(executor)
        .await?;
        Ok(user)
    }
}
