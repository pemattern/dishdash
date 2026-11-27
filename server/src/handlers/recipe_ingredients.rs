use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    payloads::{
        Many,
        recipe_ingredient::{
            CreateRecipeIngredientRequest, RecipeIngredientResponse, UpdateRecipeIngredientRequest,
        },
    },
    requests::{app_path::AppPath, validated_json::ValidatedJson},
    responses::{Delete, Get, Patch, Post},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_ingredients(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
) -> AppResult<Get<Many<RecipeIngredientResponse>>> {
    let recipe_ingredients = state
        .recipe_service
        .find_recipe_ingredients_by_recipe_id(recipe_id)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredient(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
    ValidatedJson(recipe_ingredient): ValidatedJson<CreateRecipeIngredientRequest>,
) -> AppResult<Post<RecipeIngredientResponse>> {
    let recipe_ingredient = state
        .recipe_service
        .create_recipe_ingredient(recipe_id, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredients(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<Vec<CreateRecipeIngredientRequest>>,
) -> AppResult<Post<Many<RecipeIngredientResponse>>> {
    let recipe_ingredients = state
        .recipe_service
        .create_recipe_ingredients(recipe_id, recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredient(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
    ValidatedJson(recipe_ingredient): ValidatedJson<UpdateRecipeIngredientRequest>,
) -> AppResult<Patch<RecipeIngredientResponse>> {
    let recipe_ingredient = state
        .recipe_service
        .update_recipe_ingredient(recipe_id, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredients(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<Vec<UpdateRecipeIngredientRequest>>,
) -> AppResult<Patch<Many<RecipeIngredientResponse>>> {
    let recipe_ingredients = state
        .recipe_service
        .update_recipe_ingredients(recipe_id, recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn delete_recipe_ingredient(
    State(state): State<AppState>,
    AppPath((recipe_id, ingredient_code)): AppPath<(Uuid, String)>,
) -> AppResult<Delete> {
    let recipe_ingredient = state
        .recipe_service
        .delete_recipe_ingredient(recipe_id, ingredient_code)
        .await?;
    Ok(recipe_ingredient.into())
}
