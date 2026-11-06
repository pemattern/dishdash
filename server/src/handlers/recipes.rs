use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe::schemas::{PatchRecipeSchema, PostRecipeSchema, RecipeSchema},
    requests::{app_json::ValidatedJson, uuid_path::UuidPath},
    responses::{DeleteResponse, GetManyResponse, GetResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
) -> AppResult<GetResponse<RecipeSchema>> {
    let recipe = state.recipe_service.find_recipe_by_id(id).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn get_all_recipes(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<RecipeSchema>> {
    let recipes = state.recipe_service.get_recipes().await?;
    Ok(recipes.into())
}

#[axum::debug_handler]
pub async fn post_recipe(
    State(state): State<AppState>,
    ValidatedJson(user): ValidatedJson<PostRecipeSchema>,
) -> AppResult<PostResponse<RecipeSchema>> {
    let recipe = state.recipe_service.create_recipe(user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn patch_recipe(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
    ValidatedJson(user): ValidatedJson<PatchRecipeSchema>,
) -> AppResult<PatchResponse<RecipeSchema>> {
    let recipe = state.recipe_service.update_recipe(id, user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn delete_recipe(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
) -> AppResult<DeleteResponse> {
    let recipe = state.recipe_service.delete_recipe(id).await?;
    Ok(recipe.into())
}
