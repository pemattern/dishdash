use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::AppResult, models::RecipeMealTypeModel};

pub struct RecipeMealTypes;

impl RecipeMealTypes {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: &Uuid,
    ) -> AppResult<Vec<RecipeMealTypeModel>> {
        let recipe_meal_types = sqlx::query_as!(
            RecipeMealTypeModel,
            "SELECT *
            FROM recipe_meal_types
            WHERE recipe_id = $1",
            recipe_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(recipe_meal_types)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        meal_type_code: String,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = sqlx::query_as!(
            RecipeMealTypeModel,
            "INSERT INTO recipe_meal_types (recipe_id, meal_type_code)
            VALUES ($1, $2)
            RETURNING *;",
            recipe_id,
            meal_type_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_meal_type)
    }

    pub async fn insert_multiple<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: &Uuid,
        meal_type_codes: &[String],
    ) -> AppResult<Vec<RecipeMealTypeModel>> {
        let recipe_meal_types = sqlx::query_as!(
            RecipeMealTypeModel,
            "INSERT INTO recipe_meal_types (recipe_id, meal_type_code)
            SELECT $1, unnest($2::TEXT[])
            RETURNING *;",
            recipe_id,
            meal_type_codes,
        )
        .fetch_all(executor)
        .await?;
        Ok(recipe_meal_types)
    }

    pub async fn delete<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        meal_type_code: String,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = sqlx::query_as!(
            RecipeMealTypeModel,
            "DELETE FROM recipe_meal_types
            WHERE recipe_id = $1 AND meal_type_code = $2
            RETURNING *;",
            recipe_id,
            meal_type_code,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_meal_type)
    }

    pub async fn delete_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: &Uuid,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = sqlx::query_as!(
            RecipeMealTypeModel,
            "DELETE FROM recipe_meal_types
            WHERE recipe_id = $1
            RETURNING *;",
            recipe_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_meal_type)
    }
}
