use axum::extract::State;

use crate::{
    error::AppResult,
    models::{
        ingredient::schemas::IngredientSchema, search::ingredient::schemas::IngredientSearchSchema,
    },
    requests::search_query::SearchQuery,
    responses::GetManyResponse,
    state::AppState,
};

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
    search_query: SearchQuery,
) -> AppResult<GetManyResponse<IngredientSearchSchema>> {
    let ingredients = state
        .ingredient_service
        .search_ingredients(search_query)
        .await?;
    Ok(ingredients.into())
}
