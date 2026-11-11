use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        joins::recipe_ingredient::RecipeIngredientJoinIngredient,
        recipe_ingredient::{
            RecipeIngredient,
            schemas::{PatchRecipeIngredientSchema, PostRecipeIngredientSchema},
        },
    },
};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn select_by_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_code: String,
    ) -> AppResult<RecipeIngredientJoinIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredientJoinIngredient,
            "SELECT ri.*, i.code AS ingredient_name
            FROM recipe_ingredients ri
            INNER JOIN ingredients i
            ON ri.ingredient_code = i.code
            WHERE ri.recipe_id = $1
            AND ri.ingredient_code = $2;",
            recipe_id,
            ingredient_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
    ) -> AppResult<Vec<RecipeIngredientJoinIngredient>> {
        let recipe_ingredients = sqlx::query_as!(
            RecipeIngredientJoinIngredient,
            "SELECT ri.*, i.code AS ingredient_name
            FROM recipe_ingredients ri
            INNER JOIN ingredients i
            ON ri.ingredient_code = i.code
            WHERE ri.recipe_id = $1",
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
        recipe_ingredient: PostRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
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
        recipe_ingredient: PatchRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
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

    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_code: String,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
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
