use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::RepositoryResult, models::recipe_ingredient::RecipeIngredient};

pub struct RecipeIngredients;

impl RecipeIngredients {
    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        ingredient_id: Uuid,
        unit_id: Uuid,
        quantity: f64,
    ) -> RepositoryResult<RecipeIngredient> {
        let recipe_ingredient = sqlx::query_as!(
            RecipeIngredient,
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, unit_id, quantity)
            VALUES ($1, $2, $3, $4)
            RETURNING *;",
            recipe_id,
            ingredient_id,
            unit_id,
            quantity,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }

    pub async fn update<'a>(
        executor: impl PgExecutor<'a>,
        id: Uuid,
        recipe_id: Option<Uuid>,
        ingredient_id: Option<Uuid>,
        unit_id: Option<Uuid>,
        quantity: Option<f64>,
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
            recipe_id,
            ingredient_id,
            unit_id,
            quantity,
            id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_ingredient)
    }
}
