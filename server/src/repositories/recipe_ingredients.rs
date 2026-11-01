use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::recipe_ingredient::{
        RecipeIngredient,
        join::RecipeIngredientJoinIngredient,
        schemas::{PatchRecipeIngredientSchema, PostRecipeIngredientSchema},
    },
};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
    ) -> RepositoryResult<Vec<RecipeIngredientJoinIngredient>> {
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
        recipe_ingredient: PostRecipeIngredientSchema,
    ) -> RepositoryResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, unit_id, quantity)
            VALUES ($1, $2, $3, $4)
            RETURNING *;",
            recipe_ingredient.recipe_id,
            recipe_ingredient.ingredient_id,
            recipe_ingredient.unit_id,
            recipe_ingredient.quantity,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
        recipe_ingredient: PatchRecipeIngredientSchema,
    ) -> RepositoryResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "UPDATE recipe_ingredients
            SET
                recipe_id = COALESCE($1, recipe_id),
                ingredient_id = COALESCE($2, ingredient_id),
                unit_id = COALESCE($3, unit_id),
                quantity = COALESCE($4, quantity)
            WHERE id = $5
            RETURNING *;",
            recipe_ingredient.recipe_id,
            recipe_ingredient.ingredient_id,
            recipe_ingredient.unit_id,
            recipe_ingredient.quantity,
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }
}
