use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::ingredient::schemas::{IngredientSchema, PatchIngredientSchema, PostIngredientSchema},
    requests::{app_json::AppJson, app_path::AppPath},
    responses::{DeleteResponse, GetManyResponse, GetResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_ingredient(
    State(state): State<AppState>,
    AppPath(ingredient_id): AppPath<Uuid>,
) -> AppResult<GetResponse<IngredientSchema>> {
    let ingredient = state
        .ingredient_service
        .find_ingredient_by_id(ingredient_id)
        .await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn get_ingredients(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<IngredientSchema>> {
    let ingredients = state.ingredient_service.all_ingredients().await?;
    Ok(ingredients.into())
}

#[axum::debug_handler]
pub async fn post_ingredient(
    State(state): State<AppState>,
    AppJson(user): AppJson<PostIngredientSchema>,
) -> AppResult<PostResponse<IngredientSchema>> {
    let ingredient = state.ingredient_service.create_ingredient(user).await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_ingredient(
    State(state): State<AppState>,
    AppPath(ingredient_id): AppPath<Uuid>,
    AppJson(user): AppJson<PatchIngredientSchema>,
) -> AppResult<PatchResponse<IngredientSchema>> {
    let ingredient = state
        .ingredient_service
        .update_ingredient(ingredient_id, user)
        .await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn delete_ingredient(
    State(state): State<AppState>,
    AppPath(ingredient_id): AppPath<Uuid>,
) -> AppResult<DeleteResponse> {
    let ingredient = state
        .ingredient_service
        .delete_ingredient(ingredient_id)
        .await?;
    Ok(ingredient.into())
}
