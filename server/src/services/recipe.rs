use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ServiceResult,
    models::{
        recipe::{
            Recipe,
            schemas::{PatchRecipeSchema, PostRecipeSchema},
        },
        recipe_ingredient::{
            RecipeIngredient, join::RecipeIngredientJoinIngredient,
            schemas::PostRecipeIngredientSchema,
        },
    },
    repositories::{recipe_ingredients::RecipeIngredients, recipes::Recipes},
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

    pub async fn find_recipe_by_id(&self, recipe_id: Uuid) -> ServiceResult<Option<Recipe>> {
        let recipe = Recipes::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe)
    }

    pub async fn all_recipes(&self) -> ServiceResult<Vec<Recipe>> {
        let recipes = Recipes::select_all(&self.pool).await?;
        Ok(recipes)
    }

    pub async fn find_recipe_ingredients_by_recipe_id(
        &self,
        recipe_id: Uuid,
    ) -> ServiceResult<Vec<RecipeIngredientJoinIngredient>> {
        let recipe_ingredients =
            RecipeIngredients::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe_ingredients)
    }

    pub async fn create_recipe(&self, recipe: PostRecipeSchema) -> ServiceResult<Recipe> {
        let recipe = Recipes::insert(&self.pool, recipe.name, recipe.summary).await?;
        Ok(recipe)
    }

    pub async fn create_recipe_ingredients(
        &self,
        recipe_ingredients: Vec<PostRecipeIngredientSchema>,
    ) -> ServiceResult<Vec<RecipeIngredient>> {
        let mut tx = self.pool.begin().await?;
        let mut inserted_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for recipe_ingredient in recipe_ingredients {
            let inserted_recipe_ingredient =
                RecipeIngredients::insert(&mut *tx, recipe_ingredient).await?;
            inserted_recipe_ingredients.push(inserted_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(inserted_recipe_ingredients)
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
