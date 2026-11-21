use base64::{Engine, engine::general_purpose::STANDARD};
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
        let mut image_url = None;
        if let Some(image_data) = &recipe.image_data {
            let bytes = STANDARD.decode(image_data)?;
            let path = Images::save_recipe_image(&bytes).await?;
            image_url = Some(path);
        }
        let recipe = Recipes::insert(&self.pool, &recipe, image_url).await?;
        let recipe_meal_types =
            RecipeMealTypes::select_by_recipe_id(&self.pool, &recipe.id).await?;
        Ok((recipe, recipe_meal_types).into())
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
        meal_type_code: String,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type =
            RecipeMealTypes::insert(&self.pool, recipe_id, meal_type_code).await?;
        Ok(recipe_meal_type)
    }

    pub async fn update_recipe(
        &self,
        recipe_id: Uuid,
        recipe: UpdateRecipeRequest,
    ) -> AppResult<RecipeResponse> {
        let mut image_url = None;
        if let Some(maybe_image_data) = &recipe.image_data {
            if let Some(image_data) = maybe_image_data {
                let bytes = STANDARD.decode(image_data)?;
                let path = Images::save_recipe_image(&bytes).await?;
                image_url = Some(Some(path));
            } else {
                image_url = Some(None);
            }
        }
        let recipe = tokio::try_join!(
            Recipes::update(&self.pool, recipe_id, recipe, image_url),
            RecipeMealTypes::select_by_recipe_id(&self.pool, &recipe_id),
        )?;
        Ok(recipe.into())
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
        meal_type_code: String,
    ) -> AppResult<RecipeMealTypeModel> {
        let recipe_meal_type =
            RecipeMealTypes::delete_by_id(&self.pool, recipe_id, meal_type_code).await?;
        Ok(recipe_meal_type)
    }
}
