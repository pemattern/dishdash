use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::users::{delete_user, get_me, get_user, patch_user, post_user},
    routes::EMPTY_PATH,
    state::AppState,
};

const USERS_PATH: &str = "/users";
const USER_ID_PATH: &str = "/{user_id}";
const ME_PATH: &str = "/me";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        USERS_PATH,
        Router::new()
            .route(EMPTY_PATH, post(post_user))
            .route(ME_PATH, get(get_me))
            .route(
                USER_ID_PATH,
                get(get_user).patch(patch_user).delete(delete_user),
            ),
    )
}
