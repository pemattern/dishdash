use axum::{Router, routing::get};

use crate::{
    handlers::ingredients::{
        delete_ingredient, get_all_ingredients, get_ingredient, patch_ingredient, post_ingredient,
    },
    routes::{EMPTY_PATH, ID_PATH},
    state::AppState,
};

const INGREDIENTS_PATH: &str = "/ingredients";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new()
            .route(EMPTY_PATH, get(get_all_ingredients).post(post_ingredient))
            .route(
                ID_PATH,
                get(get_ingredient)
                    .patch(patch_ingredient)
                    .delete(delete_ingredient),
            ),
    )
}
