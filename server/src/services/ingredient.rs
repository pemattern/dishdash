use sqlx::PgPool;

use crate::{
    error::AppResult,
    payloads::{Many, ingredient::IngredientResponse},
    repositories::database::Ingredients,
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

    pub async fn search_ingredients(
        &self,
        search_query: SearchQuery,
    ) -> AppResult<Many<IngredientResponse>> {
        let ingredients = Ingredients::search(&self.pool, search_query).await?;
        Ok(ingredients.into())
    }
}
