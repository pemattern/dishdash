use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe_ingredient::{
        join::schemas::RecipeIngredientJoinIngredientSchema,
        schemas::{
            PatchRecipeIngredientSchema, PostRecipeIngredientSchema, RecipeIngredientSchema,
        },
    },
    requests::UuidMap,
    responses::{GetManyResponse, PatchManyResponse, PostManyResponse},
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
    Path(recipe_id): Path<Uuid>,
    Json(recipe_ingredients): Json<UuidMap<PostRecipeIngredientSchema>>,
) -> AppResult<PostManyResponse<RecipeIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .create_recipe_ingredients(recipe_id, recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredients(
    State(state): State<AppState>,
    Path(recipe_id): Path<Uuid>,
    Json(recipe_ingredients): Json<UuidMap<PatchRecipeIngredientSchema>>,
) -> AppResult<PatchManyResponse<RecipeIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .update_recipe_ingredients(recipe_id, recipe_ingredients)
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
