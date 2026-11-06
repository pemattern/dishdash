use axum::extract::State;
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::recipe_ingredient::{
        join::schemas::RecipeIngredientJoinIngredientSchema,
        schemas::{
            PatchRecipeIngredientSchema, PostRecipeIngredientSchema, RecipeIngredientSchema,
        },
    },
    requests::{UuidMap, app_json::ValidatedJson, uuid_path::UuidPath},
    responses::{
        DeleteResponse, GetManyResponse, GetResponse, PatchManyResponse, PatchResponse,
        PostManyResponse, PostResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_ingredient(
    State(state): State<AppState>,
    UuidPath((recipe_id, ingredient_id)): UuidPath<(Uuid, Uuid)>,
) -> AppResult<GetResponse<RecipeIngredientJoinIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .find_recipe_ingredient_by_id(recipe_id, ingredient_id)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn get_recipe_ingredients(
    State(state): State<AppState>,
    UuidPath(recipe_id): UuidPath<Uuid>,
) -> AppResult<GetManyResponse<RecipeIngredientJoinIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .find_recipe_ingredients_by_recipe_id(recipe_id)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredient(
    State(state): State<AppState>,
    UuidPath((recipe_id, ingredient_id)): UuidPath<(Uuid, Uuid)>,
    ValidatedJson(recipe_ingredient): ValidatedJson<PostRecipeIngredientSchema>,
) -> AppResult<PostResponse<RecipeIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .create_recipe_ingredient(recipe_id, ingredient_id, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredients(
    State(state): State<AppState>,
    UuidPath(recipe_id): UuidPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<UuidMap<PostRecipeIngredientSchema>>,
) -> AppResult<PostManyResponse<RecipeIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .create_recipe_ingredients(recipe_id, recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredient(
    State(state): State<AppState>,
    UuidPath((recipe_id, ingredient_id)): UuidPath<(Uuid, Uuid)>,
    ValidatedJson(recipe_ingredient): ValidatedJson<PatchRecipeIngredientSchema>,
) -> AppResult<PatchResponse<RecipeIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .update_recipe_ingredient(recipe_id, ingredient_id, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredients(
    State(state): State<AppState>,
    UuidPath(recipe_id): UuidPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<UuidMap<PatchRecipeIngredientSchema>>,
) -> AppResult<PatchManyResponse<RecipeIngredientSchema>> {
    let recipe_ingredients = state
        .recipe_service
        .update_recipe_ingredients(recipe_id, recipe_ingredients)
        .await?;
    Ok(recipe_ingredients.into())
}

#[axum::debug_handler]
pub async fn delete_recipe_ingredient(
    State(state): State<AppState>,
    UuidPath((recipe_id, ingredient_id)): UuidPath<(Uuid, Uuid)>,
) -> AppResult<DeleteResponse> {
    let recipe_ingredient = state
        .recipe_service
        .delete_recipe_ingredient(recipe_id, ingredient_id)
        .await?;
    Ok(recipe_ingredient.into())
}
