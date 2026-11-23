use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{error::AppResult, models::RecipeMealTypeModel, requests::constants::MealTypeCode};

pub struct RecipeMealTypes;

impl RecipeMealTypes {
    pub async fn select_by_recipe_id<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: &Uuid,
    ) -> AppResult<Vec<RecipeMealTypeModel>> {
        let recipe_meal_types = sqlx::query_as!(
            RecipeMealTypeModel,
            r#"SELECT
                recipe_id,
                meal_type_code AS "meal_type_code: MealTypeCode"
            FROM recipe_meal_types
            WHERE recipe_id = $1"#,
            recipe_id,
        )
        .fetch_all(executor)
        .await?;
        Ok(recipe_meal_types)
    }

    pub async fn insert<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        meal_type: MealTypeCode,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = sqlx::query_as!(
            RecipeMealTypeModel,
            r#"INSERT INTO recipe_meal_types (recipe_id, meal_type_code)
            VALUES ($1, $2)
            RETURNING
                recipe_id,
                meal_type_code AS "meal_type_code: MealTypeCode";"#,
            recipe_id,
            meal_type as MealTypeCode,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_meal_type)
    }

    pub async fn insert_multiple<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: &Uuid,
        meal_type_codes: &[MealTypeCode],
    ) -> AppResult<Vec<RecipeMealTypeModel>> {
        let recipe_meal_types = sqlx::query_as!(
            RecipeMealTypeModel,
            r#"INSERT INTO recipe_meal_types (recipe_id, meal_type_code)
            SELECT $1, unnest($2::meal_type_code[])
            RETURNING
                recipe_id,
                meal_type_code AS "meal_type_code: MealTypeCode";"#,
            recipe_id,
            meal_type_codes as &[MealTypeCode],
        )
        .fetch_all(executor)
        .await?;
        Ok(recipe_meal_types)
    }

    pub async fn delete<'a>(
        executor: impl PgExecutor<'a>,
        recipe_id: Uuid,
        meal_type_code: MealTypeCode,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = sqlx::query_as!(
            RecipeMealTypeModel,
            r#"DELETE FROM recipe_meal_types
            WHERE recipe_id = $1 AND meal_type_code = $2
            RETURNING
                recipe_id,
                meal_type_code AS "meal_type_code: MealTypeCode";"#,
            recipe_id,
            meal_type_code as MealTypeCode,
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
            r#"DELETE FROM recipe_meal_types
            WHERE recipe_id = $1
            RETURNING
                recipe_id,
                meal_type_code AS "meal_type_code: MealTypeCode";"#,
            recipe_id,
        )
        .fetch_one(executor)
        .await?;
        Ok(recipe_meal_type)
    }
}
