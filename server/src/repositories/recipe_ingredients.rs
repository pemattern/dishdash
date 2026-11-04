use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe_ingredient::{
        RecipeIngredient,
        join::RecipeIngredientJoinIngredient,
        schemas::{PatchRecipeIngredientSchema, PostRecipeIngredientSchema},
    },
};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn select_by_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_id: Uuid,
    ) -> AppResult<RecipeIngredientJoinIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredientJoinIngredient,
            "SELECT ri.*, i.name AS ingredient_name
            FROM recipe_ingredients ri
            INNER JOIN ingredients i
            ON ri.ingredient_id = i.id
            WHERE ri.recipe_id = $1
            AND ri.ingredient_id = $2;",
            recipe_id,
            ingredient_id,
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
            "SELECT ri.*, i.name AS ingredient_name
            FROM recipe_ingredients ri
            INNER JOIN ingredients i
            ON ri.ingredient_id = i.id
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
        ingredient_id: Uuid,
        recipe_ingredient: PostRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, unit_id, position, quantity)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *;",
            recipe_id,
            ingredient_id,
            recipe_ingredient.unit_id,
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
        ingredient_id: Uuid,
        recipe_ingredient: PatchRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "UPDATE recipe_ingredients
            SET
                unit_id = COALESCE($1, unit_id),
                position = COALESCE($2, position),
                quantity = COALESCE($3, quantity)
            WHERE recipe_id = $4 AND ingredient_id = $5
            RETURNING *;",
            recipe_ingredient.unit_id,
            recipe_ingredient.position,
            recipe_ingredient.quantity,
            recipe_id,
            ingredient_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn delete_by_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_id: Uuid,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "DELETE FROM recipe_ingredients
            WHERE recipe_id = $1 AND ingredient_id = $2
            RETURNING *;",
            recipe_id,
            ingredient_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }
}
