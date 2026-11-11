use axum::extract::State;

use crate::{
    error::AppResult, models::ingredient::schemas::IngredientSchema, responses::GetManyResponse,
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_ingredients(
    State(state): State<AppState>,
) -> AppResult<GetManyResponse<IngredientSchema>> {
    let ingredients = state.ingredient_service.all_ingredients().await?;
    Ok(ingredients.into())
}
