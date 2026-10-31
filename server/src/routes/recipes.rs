use axum::{Router, routing::get};

use crate::{
    handlers::recipes::{delete_recipe, get_all_recipes, get_recipe, patch_recipe, post_recipe},
    routes::{EMPTY_PATH, ingredients::recipe_ingredients_router},
    state::AppState,
};

const RECIPES_PATH: &str = "/recipes";
const RECIPE_ID_PATH: &str = "/{recipe_id}";

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
                    .merge(recipe_ingredients_router()),
            ),
    )
}
