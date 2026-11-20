use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        recipe::{PatchRecipeSchema, PostRecipeSchema, Recipe},
        recipe_ingredient::{
            PatchRecipeIngredientSchema, PostRecipeIngredientSchema, RecipeIngredient,
            RecipeIngredientJoinIngredient,
        },
        recipe_meal_type::RecipeMealType,
    },
    repositories::{database::RecipeIngredients, database::RecipeMealTypes, database::Recipes},
    requests::code_map::CodeMap,
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

    pub async fn find_recipes(&self) -> AppResult<Vec<Recipe>> {
        let recipes = Recipes::select_all(&self.pool).await?;
        Ok(recipes)
    }

    pub async fn find_recipe_ingredient_by_code(
        &self,
        recipe_id: Uuid,
        ingredient_code: String,
    ) -> AppResult<RecipeIngredientJoinIngredient> {
        let recipe_ingredient =
            RecipeIngredients::select_by_id(&self.pool, recipe_id, ingredient_code).await?;
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

    pub async fn find_recipe_meal_types_by_recipe_id(
        &self,
        recipe_id: Uuid,
    ) -> AppResult<Vec<RecipeMealType>> {
        let recipe_meal_types = RecipeMealTypes::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe_meal_types)
    }

    pub async fn create_recipe(&self, recipe: PostRecipeSchema) -> AppResult<Recipe> {
        let recipe = Recipes::insert(&self.pool, recipe).await?;
        Ok(recipe)
    }

    pub async fn create_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_code: String,
        recipe_ingredient: PostRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::insert(&self.pool, recipe_id, ingredient_code, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient)
    }

    pub async fn create_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: CodeMap<PostRecipeIngredientSchema>,
    ) -> AppResult<Vec<RecipeIngredient>> {
        let mut tx = self.pool.begin().await?;
        let mut inserted_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_code, recipe_ingredient) in recipe_ingredients {
            let inserted_recipe_ingredient =
                RecipeIngredients::insert(&mut *tx, recipe_id, ingredient_code, recipe_ingredient)
                    .await?;
            inserted_recipe_ingredients.push(inserted_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(inserted_recipe_ingredients)
    }

    pub async fn create_recipe_meal_type(
        &self,
        recipe_id: Uuid,
        meal_type_code: String,
    ) -> AppResult<RecipeMealType> {
        let recipe_meal_type =
            RecipeMealTypes::insert(&self.pool, recipe_id, meal_type_code).await?;
        Ok(recipe_meal_type)
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
        ingredient_code: String,
        recipe_ingredient: PatchRecipeIngredientSchema,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::update(&self.pool, recipe_id, ingredient_code, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient)
    }

    pub async fn update_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: CodeMap<PatchRecipeIngredientSchema>,
    ) -> AppResult<Vec<RecipeIngredient>> {
        let mut tx = self.pool.begin().await?;
        let mut updated_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_code, recipe_ingredient) in recipe_ingredients {
            let updated_recipe_ingredient =
                RecipeIngredients::update(&mut *tx, recipe_id, ingredient_code, recipe_ingredient)
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
        ingredient_code: String,
    ) -> AppResult<RecipeIngredient> {
        let recipe_ingredient =
            RecipeIngredients::delete_by_id(&self.pool, recipe_id, ingredient_code).await?;
        Ok(recipe_ingredient)
    }

    pub async fn delete_recipe_meal_type(
        &self,
        recipe_id: Uuid,
        meal_type_code: String,
    ) -> AppResult<RecipeMealType> {
        let recipe_meal_type =
            RecipeMealTypes::delete_by_id(&self.pool, recipe_id, meal_type_code).await?;
        Ok(recipe_meal_type)
    }
}
