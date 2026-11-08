use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        recipe::{
            Recipe,
            schemas::{PatchRecipeSchema, PostRecipeSchema},
        },
        recipe_ingredient::{
            RecipeIngredient,
            join::RecipeIngredientJoinIngredient,
            schemas::{PatchRecipeIngredientSchema, PostRecipeIngredientSchema},
        },
    },
    repositories::{recipe_ingredients::RecipeIngredients, recipes::Recipes},
    requests::uuid_set::UuidSet,
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

    pub async fn find_recipe_by_id(&self, recipe_id: Uuid) -> AppResult<Recipe> {
        let recipe = Recipes::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe)
    }

    pub async fn get_recipes(&self) -> AppResult<Vec<Recipe>> {
        let recipes = Recipes::select_all(&self.pool).await?;
        Ok(recipes)
    }

    pub async fn find_recipe_ingredient_by_id(
        &self,
        recipe_id: Uuid,
        ingredient_id: Uuid,
    ) -> AppResult<RecipeIngredientJoinIngredient> {
        let recipe_ingredient =
            RecipeIngredients::select_by_id(&self.pool, recipe_id, ingredient_id).await?;
        Ok(recipe_ingredient)
    }

    pub async fn find_recipe_ingredients_by_recipe_id(
        &self,
        recipe_id: Uuid,
    ) -> AppResult<Vec<RecipeIngredientJoinIngredient>> {
        let recipe_ingredients =
            RecipeIngredients::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe_ingredients)
    }

    pub async fn create_recipe(&self, recipe: PostRecipeSchema) -> AppResult<Recipe> {
        let recipe = Recipes::insert(&self.pool, recipe).await?;
        Ok(recipe)
    }

    pub async fn create_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_id: Uuid,
        recipe_ingredient: PostRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::insert(&self.pool, recipe_id, ingredient_id, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient)
    }

    pub async fn create_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: UuidSet<PostRecipeIngredientSchema>,
    ) -> AppResult<Vec<RecipeIngredient>> {
        let mut tx = self.pool.begin().await?;
        let mut inserted_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_id, recipe_ingredient) in recipe_ingredients.map {
            let inserted_recipe_ingredient =
                RecipeIngredients::insert(&mut *tx, recipe_id, ingredient_id, recipe_ingredient)
                    .await?;
            inserted_recipe_ingredients.push(inserted_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(inserted_recipe_ingredients)
    }

    pub async fn update_recipe(
        &self,
        recipe_id: Uuid,
        recipe: PatchRecipeSchema,
    ) -> AppResult<Recipe> {
        let recipe = Recipes::update(&self.pool, recipe_id, recipe).await?;
        Ok(recipe)
    }

    pub async fn update_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_id: Uuid,
        recipe_ingredient: PatchRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::update(&self.pool, recipe_id, ingredient_id, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient)
    }

    pub async fn update_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: UuidSet<PatchRecipeIngredientSchema>,
    ) -> AppResult<Vec<RecipeIngredient>> {
        let mut tx = self.pool.begin().await?;
        let mut updated_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_id, recipe_ingredient) in recipe_ingredients {
            let updated_recipe_ingredient =
                RecipeIngredients::update(&mut *tx, recipe_id, ingredient_id, recipe_ingredient)
                    .await?;
            updated_recipe_ingredients.push(updated_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(updated_recipe_ingredients)
    }

    pub async fn delete_recipe(&self, recipe_id: Uuid) -> AppResult<Recipe> {
        let recipe = Recipes::delete_by_id(&self.pool, recipe_id).await?;
        Ok(recipe)
    }

    pub async fn delete_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_id: Uuid,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::delete_by_id(&self.pool, recipe_id, ingredient_id).await?;
        Ok(recipe_ingredient)
    }
}
