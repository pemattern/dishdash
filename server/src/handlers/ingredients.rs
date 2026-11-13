use axum::extract::{Query, State};
use serde::Deserialize;

use crate::{
    error::AppResult, models::ingredient::schemas::IngredientSchema, responses::GetManyResponse,
    state::AppState,
};

#[derive(Deserialize)]
pub struct SearchIngredientsQuery {
    pub q: String,
}

#[axum::debug_handler]
pub async fn get_ingredients(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<IngredientSchema>> {
    let ingredients = state.ingredient_service.all_ingredients().await?;
    Ok(ingredients.into())
}

#[axum::debug_handler]
pub async fn search_ingredients(
    State(state): State<AppState>,
    Query(query): Query<SearchIngredientsQuery>,
) -> AppResult<GetManyResponse<IngredientSchema>> {
    let ingredients = state.ingredient_service.search_ingredients(query).await?;
    Ok(ingredients.into())
}
