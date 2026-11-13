use sqlx::PgPool;

use crate::{
    error::AppResult,
    models::{ingredient::Ingredient, search::ingredient::IngredientSearch},
    repositories::ingredients::Ingredients,
    requests::search_query::SearchQuery,
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
        search_query: SearchQuery,
    ) -> AppResult<Vec<IngredientSearch>> {
        let ingredients = Ingredients::search(&self.pool, search_query).await?;
        Ok(ingredients)
    }
}
