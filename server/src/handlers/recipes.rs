use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe::{PatchRecipeSchema, PostRecipeSchema, RecipeSchema},
    requests::{app_path::AppPath, validated_json::ValidatedJson},
    responses::{DeleteResponse, GetManyResponse, GetResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<GetResponse<RecipeSchema>> {
    let recipe = state.recipe_service.find_recipe_by_id(id).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn get_all_recipes(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<RecipeSchema>> {
    let recipes = state.recipe_service.find_recipes().await?;
    Ok(recipes.into())
}

#[axum::debug_handler]
pub async fn post_recipe(
    State(state): State<AppState>,
    ValidatedJson(recipe): ValidatedJson<PostRecipeSchema>,
) -> AppResult<PostResponse<RecipeSchema>> {
    let recipe = state.recipe_service.create_recipe(recipe).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn patch_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
    ValidatedJson(recipe): ValidatedJson<PatchRecipeSchema>,
) -> AppResult<PatchResponse<RecipeSchema>> {
    let recipe = state.recipe_service.update_recipe(id, recipe).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn delete_recipe(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<DeleteResponse> {
    let recipe = state.recipe_service.delete_recipe(id).await?;
    Ok(recipe.into())
}
