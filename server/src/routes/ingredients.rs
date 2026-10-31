use axum::{Router, routing::get};

use crate::{
    handlers::ingredients::{
        delete_ingredient, get_all_ingredients, get_ingredient, patch_ingredient, post_ingredient,
    },
    routes::EMPTY_PATH,
    state::AppState,
};

const INGREDIENTS_PATH: &str = "/ingredients";
const INGREDIENT_ID_PATH: &str = "/{ingredient_id}";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new()
            .route(EMPTY_PATH, get(get_all_ingredients).post(post_ingredient))
            .route(
                INGREDIENT_ID_PATH,
                get(get_ingredient)
                    .patch(patch_ingredient)
                    .delete(delete_ingredient),
            ),
    )
}

pub fn recipe_ingredients_router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new()
            .route(EMPTY_PATH, get(get_all_recipes).post(post_recipe))
            .route(
                INGREDIENT_ID_PATH,
                get(get_recipe).patch(patch_recipe).delete(delete_recipe),
            ),
    )
}
