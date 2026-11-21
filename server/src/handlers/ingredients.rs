use axum::extract::State;

use crate::{
    error::AppResult,
    payloads::{Many, ingredient::IngredientResponse},
    requests::search_query::SearchQuery,
    responses::Get,
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_ingredients(
    State(state): State<AppState>,
    search_query: SearchQuery,
) -> AppResult<Get<Many<IngredientResponse>>> {
    let ingredients = state
        .ingredient_service
        .search_ingredients(search_query)
        .await?;
    Ok(ingredients.into())
}
