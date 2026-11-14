use axum::{Router, routing::get};

use crate::{
    handlers::{
        ingredients::search_ingredients,
        recipe_ingredients::{
            delete_recipe_ingredient, get_recipe_ingredient, get_recipe_ingredients,
            patch_recipe_ingredient, patch_recipe_ingredients, post_recipe_ingredient,
            post_recipe_ingredients,
        },
    },
    routes::EMPTY_PATH,
    state::AppState,
};

const INGREDIENTS_PATH: &str = "/ingredients";
const INGREDIENT_ID_PATH: &str = "/{ingredient_id}";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new().route(EMPTY_PATH, get(search_ingredients)),
    )
}

pub fn recipe_ingredients_router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new()
            .route(
                EMPTY_PATH,
                get(get_recipe_ingredients)
                    .post(post_recipe_ingredients)
                    .patch(patch_recipe_ingredients),
            )
            .route(
                INGREDIENT_ID_PATH,
                get(get_recipe_ingredient)
                    .post(post_recipe_ingredient)
                    .patch(patch_recipe_ingredient)
                    .delete(delete_recipe_ingredient),
            ),
    )
}
