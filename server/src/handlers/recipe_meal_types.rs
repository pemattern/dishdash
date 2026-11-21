use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe_meal_type::RecipeMealTypeSchema,
    requests::app_path::AppPath,
    responses::{Delete, GetManyResponse, Post},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_meal_types(
    State(state): State<AppState>,
    AppPath(recipe_id): AppPath<Uuid>,
) -> AppResult<GetManyResponse<RecipeMealTypeSchema>> {
    let recipe_meal_types = state
        .recipe_service
        .find_recipe_meal_types_by_recipe_id(recipe_id)
        .await?;
    Ok(recipe_meal_types.into())
}

#[axum::debug_handler]
pub async fn post_recipe_meal_type(
    State(state): State<AppState>,
    AppPath((recipe_id, meal_type_code)): AppPath<(Uuid, String)>,
) -> AppResult<Post<RecipeMealTypeSchema>> {
    let recipe_meal_type = state
        .recipe_service
        .create_recipe_meal_type(recipe_id, meal_type_code)
        .await?;
    Ok(recipe_meal_type.into())
}

#[axum::debug_handler]
pub async fn delete_recipe_meal_type(
    State(state): State<AppState>,
    AppPath((recipe_id, meal_type_code)): AppPath<(Uuid, String)>,
) -> AppResult<Delete> {
    let recipe_meal_type = state
        .recipe_service
        .delete_recipe_ingredient(recipe_id, meal_type_code)
        .await?;
    Ok(recipe_meal_type.into())
}
