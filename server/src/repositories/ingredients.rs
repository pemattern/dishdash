use sqlx::PgExecutor;

use crate::{
    error::AppResult, handlers::ingredients::SearchIngredientsQuery, models::ingredient::Ingredient,
};

pub struct Ingredients;

impl Ingredients {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<Ingredient>> {
        let ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients;")
            .fetch_all(executor)
            .await?;
        Ok(ingredients)
    }

    pub async fn search<'a>(
        executor: impl PgExecutor<'a>,
        _query: SearchIngredientsQuery,
    ) -> AppResult<Vec<Ingredient>> {
        let ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients")
            .fetch_all(executor)
            .await?;
        Ok(ingredients)
    }
}
