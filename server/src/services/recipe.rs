use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ServiceResult,
    models::recipe::{
        Recipe,
        schemas::{PatchRecipeSchema, PostRecipeSchema},
    },
    repositories::recipes::Recipes,
};

#[derive(Clone)]
pub struct RecipeService {
    pool: PgPool,
}

impl RecipeService {
    pub fn new(pool: &PgPool) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn find_recipe_by_id(&self, id: Uuid) -> ServiceResult<Option<Recipe>> {
        let recipe = Recipes::select_by_id(&self.pool, id).await?;
        Ok(recipe)
    }

    pub async fn all_recipes(&self) -> ServiceResult<Vec<Recipe>> {
        let recipes = Recipes::select_all(&self.pool).await?;
        Ok(recipes)
    }

    pub async fn create_recipe(&self, recipe: PostRecipeSchema) -> ServiceResult<Recipe> {
        let recipe = Recipes::insert(&self.pool, recipe.name, recipe.summary).await?;
        Ok(recipe)
    }

    pub async fn update_recipe(
        &self,
        id: Uuid,
        recipe: PatchRecipeSchema,
    ) -> ServiceResult<Recipe> {
        let recipe = Recipes::update(&self.pool, id, recipe.name, recipe.summary).await?;
        Ok(recipe)
    }

    pub async fn delete_recipe(&self, id: Uuid) -> ServiceResult<Recipe> {
        let recipe = Recipes::delete_by_id(&self.pool, id).await?;
        Ok(recipe)
    }
}
