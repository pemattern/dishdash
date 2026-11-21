use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::RecipeIngredientModel,
    payloads::recipe_ingredient::{CreateRecipeIngredientRequest, UpdateRecipeIngredientRequest},
};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
    ) -> AppResult<Vec<RecipeIngredientModel>> {
        let recipe_ingredients = sqlx::query_as!(
            RecipeIngredientModel,
            "SELECT * FROM recipe_ingredients
            WHERE recipe_id = $1;",
            recipe_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(recipe_ingredients)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_code: String,
        recipe_ingredient: CreateRecipeIngredientRequest,
    ) -> AppResult<RecipeIngredientModel> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredientModel,
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_code, unit_code, position, quantity)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *;",
            recipe_id,
            ingredient_code,
            recipe_ingredient.unit_code,
            recipe_ingredient.position,
            recipe_ingredient.quantity,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_code: String,
        recipe_ingredient: UpdateRecipeIngredientRequest,
    ) -> AppResult<RecipeIngredientModel> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredientModel,
            "UPDATE recipe_ingredients
            SET
                unit_code = COALESCE($1, unit_code),
                position = COALESCE($2, position),
                quantity = COALESCE($3, quantity)
            WHERE recipe_id = $4 AND ingredient_code = $5
            RETURNING *;",
            recipe_ingredient.unit_code,
            recipe_ingredient.position,
            recipe_ingredient.quantity,
            recipe_id,
            ingredient_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn delete<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_code: String,
    ) -> AppResult<RecipeIngredientModel> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredientModel,
            "DELETE FROM recipe_ingredients
            WHERE recipe_id = $1 AND ingredient_code = $2
            RETURNING *;",
            recipe_id,
            ingredient_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }
}
