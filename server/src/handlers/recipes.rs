use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    payloads::{
        Many,
        recipe::{CreateRecipeRequest, RecipeResponse, UpdateRecipeRequest},
    },
    requests::{app_path::AppPath, validated_json::ValidatedJson},
    responses::{Delete, Get, Patch, Post},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<Get<RecipeResponse>> {
    let recipe = state.recipe_service.find_recipe_by_id(id).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn get_all_recipes(
    State(state): State<AppState>,
) -> AppResult<Get<Many<RecipeResponse>>> {
    let recipes = state.recipe_service.find_recipes().await?;
    Ok(recipes.into())
}

#[axum::debug_handler]
pub async fn post_recipe(
    State(state): State<AppState>,
    ValidatedJson(recipe): ValidatedJson<CreateRecipeRequest>,
) -> AppResult<Post<RecipeResponse>> {
    let recipe = state.recipe_service.create_recipe(recipe).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn patch_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
    ValidatedJson(recipe): ValidatedJson<UpdateRecipeRequest>,
) -> AppResult<Patch<RecipeResponse>> {
    let recipe = state.recipe_service.update_recipe(id, recipe).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn delete_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<Delete> {
    let recipe = state.recipe_service.delete_recipe(id).await?;
    Ok(recipe.into())
}
