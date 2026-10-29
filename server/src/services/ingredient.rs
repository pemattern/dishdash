use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ServiceResult,
    models::ingredient::Ingredient,
    repositories::ingredients::Ingredients,
    schemas::ingredient::{PatchIngredientRequest, PostIngredientRequest},
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

    pub async fn find_ingredient_by_id(&self, id: Uuid) -> ServiceResult<Option<Ingredient>> {
        let ingredient = Ingredients::select_by_id(&self.pool, id).await?;
        Ok(ingredient)
    }

    pub async fn create_ingredient(
        &self,
        ingredient: PostIngredientRequest,
    ) -> ServiceResult<Ingredient> {
        let ingredient = Ingredients::insert(&self.pool, ingredient.name).await?;
        Ok(ingredient)
    }

    pub async fn update_ingredient(
        &self,
        id: Uuid,
        ingredient: PatchIngredientRequest,
    ) -> ServiceResult<Ingredient> {
        let user = Ingredients::update(&self.pool, id, ingredient.name).await?;
        Ok(user)
    }

    pub async fn delete_ingredient(&self, id: Uuid) -> ServiceResult<Ingredient> {
        let ingredient = Ingredients::delete_by_id(&self.pool, id).await?;
        Ok(ingredient)
    }
}
