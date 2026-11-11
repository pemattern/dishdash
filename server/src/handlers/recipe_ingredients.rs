use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        joins::recipe_ingredient::schemas::RecipeIngredientJoinIngredientSchema,
        recipe_ingredient::schemas::{
            PatchRecipeIngredientSchema, PostRecipeIngredientSchema, RecipeIngredientSchema,
        },
    },
    requests::{code_map::CodeMap, uuid_path::UuidPath, validated_json::ValidatedJson},
    responses::{
        DeleteResponse, GetManyResponse, GetResponse, PatchManyResponse, PatchResponse,
        PostManyResponse, PostResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_recipe_ingredient(
    State(state): State<AppState>,
    Path((recipe_id, ingredient_code)): Path<(Uuid, String)>,
) -> AppResult<GetResponse<RecipeIngredientJoinIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .find_recipe_ingredient_by_code(recipe_id, ingredient_code)
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
    Path((recipe_id, ingredient_code)): Path<(Uuid, String)>,
    ValidatedJson(recipe_ingredient): ValidatedJson<PostRecipeIngredientSchema>,
) -> AppResult<PostResponse<RecipeIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .create_recipe_ingredient(recipe_id, ingredient_code, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn post_recipe_ingredients(
    State(state): State<AppState>,
    UuidPath(recipe_id): UuidPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<CodeMap<PostRecipeIngredientSchema>>,
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
    Path((recipe_id, ingredient_code)): Path<(Uuid, String)>,
    ValidatedJson(recipe_ingredient): ValidatedJson<PatchRecipeIngredientSchema>,
) -> AppResult<PatchResponse<RecipeIngredientSchema>> {
    let recipe_ingredient = state
        .recipe_service
        .update_recipe_ingredient(recipe_id, ingredient_code, recipe_ingredient)
        .await?;
    Ok(recipe_ingredient.into())
}

#[axum::debug_handler]
pub async fn patch_recipe_ingredients(
    State(state): State<AppState>,
    UuidPath(recipe_id): UuidPath<Uuid>,
    ValidatedJson(recipe_ingredients): ValidatedJson<CodeMap<PatchRecipeIngredientSchema>>,
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
    Path((recipe_id, ingredient_code)): Path<(Uuid, String)>,
) -> AppResult<DeleteResponse> {
    let recipe_ingredient = state
        .recipe_service
        .delete_recipe_ingredient(recipe_id, ingredient_code)
        .await?;
    Ok(recipe_ingredient.into())
}
