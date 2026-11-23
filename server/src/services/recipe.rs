use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::RecipeMealTypeModel,
    payloads::{
        Many,
        recipe::{CreateRecipeRequest, RecipeResponse, UpdateRecipeRequest},
        recipe_ingredient::{
            CreateRecipeIngredientRequest, RecipeIngredientResponse, UpdateRecipeIngredientRequest,
        },
    },
    repositories::{
        database::{RecipeIngredients, RecipeMealTypes, Recipes},
        file_system::Images,
    },
    requests::{code_map::CodeMap, constants::MealTypeCode},
    services::{map_double_option_async, map_option_async},
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

    pub async fn find_recipe_by_id(&self, recipe_id: Uuid) -> AppResult<RecipeResponse> {
        let recipe = tokio::try_join!(
            Recipes::select_by_recipe_id(&self.pool, recipe_id),
            RecipeMealTypes::select_by_recipe_id(&self.pool, &recipe_id),
        )?;
        Ok(recipe.into())
    }

    pub async fn find_recipes(&self) -> AppResult<Many<RecipeResponse>> {
        let mut response: Vec<RecipeResponse> = Vec::new();
        let recipes = Recipes::select_all(&self.pool).await?;
        for recipe in recipes {
            let recipe_meal_types =
                RecipeMealTypes::select_by_recipe_id(&self.pool, &recipe.id).await?;
            response.push((recipe, recipe_meal_types).into())
        }
        Ok(response.into())
    }

    pub async fn find_recipe_ingredients_by_recipe_id(
        &self,
        recipe_id: Uuid,
    ) -> AppResult<Many<RecipeIngredientResponse>> {
        let recipe_ingredients =
            RecipeIngredients::select_by_recipe_id(&self.pool, recipe_id).await?;
        Ok(recipe_ingredients.into())
    }

    pub async fn create_recipe(&self, recipe: CreateRecipeRequest) -> AppResult<RecipeResponse> {
        let image_url = map_option_async(&recipe.image_data, Images::save_recipe_image).await?;
        let mut tx = self.pool.begin().await?;
        let inserted_recipe = Recipes::insert(&mut *tx, &recipe, image_url).await?;
        let recipe_meal_types =
            RecipeMealTypes::insert_multiple(&mut *tx, &inserted_recipe.id, &recipe.meal_types)
                .await?;
        tx.commit().await?;
        Ok((inserted_recipe, recipe_meal_types).into())
    }

    pub async fn create_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_code: String,
        recipe_ingredient: CreateRecipeIngredientRequest,
    ) -> AppResult<RecipeIngredientResponse> {
        let recipe_ingredient =
            RecipeIngredients::insert(&self.pool, recipe_id, ingredient_code, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient.into())
    }

    pub async fn create_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: CodeMap<CreateRecipeIngredientRequest>,
    ) -> AppResult<Many<RecipeIngredientResponse>> {
        let mut tx = self.pool.begin().await?;
        let mut inserted_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_code, recipe_ingredient) in recipe_ingredients {
            let inserted_recipe_ingredient =
                RecipeIngredients::insert(&mut *tx, recipe_id, ingredient_code, recipe_ingredient)
                    .await?;
            inserted_recipe_ingredients.push(inserted_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(inserted_recipe_ingredients.into())
    }

    pub async fn create_recipe_meal_type(
        &self,
        recipe_id: Uuid,
        meal_type: MealTypeCode,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = RecipeMealTypes::insert(&self.pool, recipe_id, meal_type).await?;
        Ok(recipe_meal_type)
    }

    pub async fn update_recipe(
        &self,
        recipe_id: Uuid,
        recipe_request: UpdateRecipeRequest,
    ) -> AppResult<RecipeResponse> {
        let image_url =
            map_double_option_async(&recipe_request.image_data, Images::save_recipe_image).await?;
        let mut tx = self.pool.begin().await?;
        let recipe = Recipes::update(&mut *tx, &recipe_id, &recipe_request, image_url).await?;
        let recipe_meal_types;
        if let Some(recipe_request_meal_types) = recipe_request.meal_types {
            let _ = RecipeMealTypes::delete_by_recipe_id(&mut *tx, &recipe_id).await?;
            recipe_meal_types =
                RecipeMealTypes::insert_multiple(&mut *tx, &recipe_id, &recipe_request_meal_types)
                    .await?;
        } else {
            recipe_meal_types = RecipeMealTypes::select_by_recipe_id(&mut *tx, &recipe_id).await?;
        }
        tx.commit().await?;
        Ok((recipe, recipe_meal_types).into())
    }

    pub async fn update_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_code: String,
        recipe_ingredient: UpdateRecipeIngredientRequest,
    ) -> AppResult<RecipeIngredientResponse> {
        let recipe_ingredient =
            RecipeIngredients::update(&self.pool, recipe_id, ingredient_code, recipe_ingredient)
                .await?;
        Ok(recipe_ingredient.into())
    }

    pub async fn update_recipe_ingredients(
        &self,
        recipe_id: Uuid,
        recipe_ingredients: CodeMap<UpdateRecipeIngredientRequest>,
    ) -> AppResult<Many<RecipeIngredientResponse>> {
        let mut tx = self.pool.begin().await?;
        let mut updated_recipe_ingredients = Vec::with_capacity(recipe_ingredients.len());
        for (ingredient_code, recipe_ingredient) in recipe_ingredients {
            let updated_recipe_ingredient =
                RecipeIngredients::update(&mut *tx, recipe_id, ingredient_code, recipe_ingredient)
                    .await?;
            updated_recipe_ingredients.push(updated_recipe_ingredient);
        }
        tx.commit().await?;
        Ok(updated_recipe_ingredients.into())
    }

    pub async fn delete_recipe(&self, recipe_id: Uuid) -> AppResult<()> {
        let _ = Recipes::delete(&self.pool, recipe_id).await?;
        Ok(())
    }

    pub async fn delete_recipe_ingredient(
        &self,
        recipe_id: Uuid,
        ingredient_code: String,
    ) -> AppResult<RecipeIngredientResponse> {
        let recipe_ingredient =
            RecipeIngredients::delete(&self.pool, recipe_id, ingredient_code).await?;
        Ok(recipe_ingredient.into())
    }

    pub async fn delete_recipe_meal_type(
        &self,
        recipe_id: Uuid,
        meal_type: MealTypeCode,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type = RecipeMealTypes::delete(&self.pool, recipe_id, meal_type).await?;
        Ok(recipe_meal_type)
    }
}
