use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    models::ingredient::schemas::{IngredientSchema, PatchIngredientSchema, PostIngredientSchema},
    schemas::{DeleteResponse, GetManyResponse, GetOneResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<GetOneResponse<IngredientSchema>> {
    let ingredient = state
        .ingredient_service
        .find_ingredient_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn get_all_ingredients(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<IngredientSchema>> {
    let ingredients = state.ingredient_service.all_ingredients().await?;
    Ok(ingredients.into())
}

#[axum::debug_handler]
pub async fn post_ingredient(
    State(state): State<AppState>,
    Json(user): Json<PostIngredientSchema>,
) -> AppResult<PostResponse<IngredientSchema>> {
    let ingredient = state.ingredient_service.create_ingredient(user).await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user): Json<PatchIngredientSchema>,
) -> AppResult<PatchResponse<IngredientSchema>> {
    let ingredient = state.ingredient_service.update_ingredient(id, user).await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn delete_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<DeleteResponse> {
    let ingredient = state.ingredient_service.delete_ingredient(id).await?;
    Ok(ingredient.into())
}
