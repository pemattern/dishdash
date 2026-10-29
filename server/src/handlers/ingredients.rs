use crate::{
    error::{AppResult, ClientError},
    schemas::{
        DeleteResponse, GetResponse, PatchResponse, PostResponse,
        ingredient::{IngredientResponse, PatchIngredientRequest, PostIngredientRequest},
    },
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn get_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<GetResponse<IngredientResponse>> {
    let ingredient = state
        .ingredient_service
        .find_ingredient_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn post_ingredient(
    State(state): State<AppState>,
    Json(user): Json<PostIngredientRequest>,
) -> AppResult<PostResponse<IngredientResponse>> {
    let ingredient = state.ingredient_service.create_ingredient(user).await?;
    Ok(ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_ingredient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user): Json<PatchIngredientRequest>,
) -> AppResult<PatchResponse<IngredientResponse>> {
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
