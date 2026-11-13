use sqlx::PgPool;

use crate::{
    error::AppResult, handlers::ingredients::SearchIngredientsQuery,
    models::ingredient::Ingredient, repositories::ingredients::Ingredients,
};

#[derive(Clone)]
pub struct IngredientService {
    pool: PgPool,
}

impl IngredientService {
    pub fn new(pool: &PgPool) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn all_ingredients(&self) -> AppResult<Vec<Ingredient>> {
        let ingredients = Ingredients::select_all(&self.pool).await?;
        Ok(ingredients)
    }

    pub async fn search_ingredients(
        &self,
        query: SearchIngredientsQuery,
    ) -> AppResult<Vec<Ingredient>> {
        let ingredients = Ingredients::search(&self.pool, query).await?;
        Ok(ingredients)
    }
}
