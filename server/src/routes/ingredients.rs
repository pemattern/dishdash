use crate::{
    handlers::ingredients::{delete_ingredient, get_ingredient, patch_ingredient, post_ingredient},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

const INGREDIENTS_PATH: &str = "/users";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        INGREDIENTS_PATH,
        Router::new().route("", post(post_ingredient)).route(
            "/{uuid}",
            get(get_ingredient)
                .patch(patch_ingredient)
                .delete(delete_ingredient),
        ),
    )
}
