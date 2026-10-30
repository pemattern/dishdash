use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    models::recipe::schemas::{PatchRecipeSchema, PostRecipeSchema, RecipeSchema},
    responses::{DeleteResponse, GetManyResponse, GetOneResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<GetOneResponse<RecipeSchema>> {
    let recipe = state
        .recipe_service
        .find_recipe_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn get_all_recipes(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<RecipeSchema>> {
    let recipes = state.recipe_service.all_recipes().await?;
    Ok(recipes.into())
}

#[axum::debug_handler]
pub async fn post_recipe(
    State(state): State<AppState>,
    Json(user): Json<PostRecipeSchema>,
) -> AppResult<PostResponse<RecipeSchema>> {
    let recipe = state.recipe_service.create_recipe(user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn patch_recipe(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user): Json<PatchRecipeSchema>,
) -> AppResult<PatchResponse<RecipeSchema>> {
    let recipe = state.recipe_service.update_recipe(id, user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn delete_recipe(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<DeleteResponse> {
    let recipe = state.recipe_service.delete_recipe(id).await?;
    Ok(recipe.into())
}
