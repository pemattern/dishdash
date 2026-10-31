use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    models::recipe::schemas::{PatchRecipeSchema, PostRecipeSchema, RecipeSchema},
    responses::{DeleteResponse, GetManyResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_ingredients(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> AppResult<GetManyResponse<RecipeSchema>> {
    let recipe = state
        .recipe_service
        .find_recipe_by_id(recipe_id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredients(
    State(state): State<AppState>,
    Json(user): Json<PostRecipeSchema>,
) -> AppResult<PostResponse<RecipeSchema>> {
    let recipe = state.recipe_service.create_recipe(user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredient(
    State(state): State<AppState>,
    Path((recipe_id, ingredient_id)): Path<(Uuid, Uuid)>,
    Json(user): Json<PatchRecipeSchema>,
) -> AppResult<PatchResponse<RecipeSchema>> {
    let recipe = state.recipe_service.update_recipe(id, user).await?;
    Ok(recipe.into())
}

#[axum::debug_handler]
pub async fn delete_recipe_ingredient(
    State(state): State<AppState>,
    Path((recipe_id, ingredient_id)): Path<(Uuid, Uuid)>,
) -> AppResult<DeleteResponse> {
    let recipe = state.recipe_service.delete_recipe(id).await?;
    Ok(recipe.into())
}
