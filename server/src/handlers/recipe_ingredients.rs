use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        recipe::schemas::{PatchRecipeSchema, RecipeSchema},
        recipe_ingredient::{
            join::schemas::RecipeIngredientJoinIngredientSchema,
            schemas::{PostRecipeIngredientSchema, RecipeIngredientSchema},
        },
    },
    responses::{DeleteResponse, GetManyResponse, PatchResponse, PostManyResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_ingredients_by_recipe_id(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> AppResult<GetManyResponse<RecipeIngredientJoinIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .find_recipe_ingredients_by_recipe_id(recipe_id)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredients(
    State(state): State<AppState>,
    Json(recipe_ingredients): Json<Vec<PostRecipeIngredientSchema>>,
) -> AppResult<PostManyResponse<RecipeIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .create_recipe_ingredients(recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredients(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
    Json(recipe_ingredients): Json<HashMap<Uuid, PatchRecipeSchema>>,
) -> AppResult<PatchResponse<RecipeSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .update_recipe_ingredients(recipe_id, recipe_ingredient)
        .await?;
    Ok(recipe_ingredients.into())
}

// #[axum::debug_handler]
// pub async fn delete_recipe_ingredient(
//     State(state): State<AppState>,
//     Path((recipe_id, ingredient_id)): Path<(Uuid, Uuid)>,
// ) -> AppResult<DeleteResponse> {
//     let recipe = state.recipe_service.delete_recipe(id).await?;
//     Ok(recipe.into())
// }
