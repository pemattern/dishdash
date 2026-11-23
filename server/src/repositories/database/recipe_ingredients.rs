use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::RecipeIngredientModel,
    payloads::recipe_ingredient::{CreateRecipeIngredientRequest, UpdateRecipeIngredientRequest},
    requests::constants::{UnitCode, UnitSystemCode},
};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
    ) -> AppResult<Vec<RecipeIngredientModel>> {
        let recipe_ingredients = sqlx::query_as!(
            RecipeIngredientModel,
            r#"SELECT
                created_at,
                updated_at,
                recipe_id,
                ingredient_code,
                unit_code AS "unit_code: UnitCode",
                unit_system_code AS "unit_system_code: UnitSystemCode",
                position,
                quantity
            FROM recipe_ingredients
            WHERE recipe_id = $1;"#,
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
            r#"INSERT INTO recipe_ingredients (recipe_id, ingredient_code, unit_code, unit_system_code, position, quantity)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                created_at,
                updated_at,
                recipe_id,
                ingredient_code,
                unit_code AS "unit_code: UnitCode",
                unit_system_code AS "unit_system_code: UnitSystemCode",
                position,
                quantity;"#,
            recipe_id,
            ingredient_code,
            recipe_ingredient.unit_code as UnitCode,
            recipe_ingredient.unit_system_code as UnitSystemCode,
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
            r#"UPDATE recipe_ingredients
            SET
                unit_code = COALESCE($1, unit_code),
                unit_system_code = COALESCE($2, unit_system_code),
                position = COALESCE($3, position),
                quantity = COALESCE($4, quantity)
            WHERE recipe_id = $5 AND ingredient_code = $6
            RETURNING
                created_at,
                updated_at,
                recipe_id,
                ingredient_code,
                unit_code AS "unit_code: UnitCode",
                unit_system_code AS "unit_system_code: UnitSystemCode",
                position,
                quantity;"#,
            recipe_ingredient.unit_code as Option<UnitCode>,
            recipe_ingredient.unit_system_code as Option<UnitSystemCode>,
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
            r#"DELETE FROM recipe_ingredients
            WHERE recipe_id = $1 AND ingredient_code = $2
            RETURNING
                created_at,
                updated_at,
                recipe_id,
                ingredient_code,
                unit_code AS "unit_code: UnitCode",
                unit_system_code AS "unit_system_code: UnitSystemCode",
                position,
                quantity;"#,
            recipe_id,
            ingredient_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }
}
