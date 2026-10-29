use crate::{
    handlers::users::{delete_user, get_me, get_user, patch_user, post_user},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

const USERS_PATH: &str = "/users";

pub fn router() -> Router<AppState> {
    Router::new().nest(
        USERS_PATH,
        Router::new()
            .route("", post(post_user))
            .route("/me", get(get_me))
            .route(
                "/{uuid}",
                get(get_user).patch(patch_user).delete(delete_user),
            ),
    )
}
