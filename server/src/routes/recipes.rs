use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{
        recipe_meal_types::{
            delete_recipe_meal_type, get_recipe_meal_types, post_recipe_meal_type,
        },
        recipes::{delete_recipe, get_all_recipes, get_recipe, patch_recipe, post_recipe},
    },
    routes::{EMPTY_PATH, ingredients::recipe_ingredients_router},
    state::AppState,
};

const RECIPES_PATH: &str = "/recipes";
const RECIPE_ID_PATH: &str = "/{recipe_id}";
const MEAL_TYPES_PATH: &str = "/meal-types";
const MEAL_TYPE_CODE_PATH: &str = "/{meal_type_code}";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        RECIPES_PATH,
        Router::new()
            .route(EMPTY_PATH, get(get_all_recipes).post(post_recipe))
            .nest(
                RECIPE_ID_PATH,
                Router::new()
                    .route(
                        EMPTY_PATH,
                        get(get_recipe).patch(patch_recipe).delete(delete_recipe),
                    )
                    .merge(recipe_ingredients_router())
                    .merge(recipe_meal_types_router()),
            ),
    )
}

fn recipe_meal_types_router() -> Router<AppState> {
    Router::new().nest(
        MEAL_TYPES_PATH,
        Router::new()
            .route(EMPTY_PATH, get(get_recipe_meal_types))
            .route(
                MEAL_TYPE_CODE_PATH,
                post(post_recipe_meal_type).delete(delete_recipe_meal_type),
            ),
    )
}
