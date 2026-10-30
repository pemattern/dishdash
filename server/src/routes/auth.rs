use axum::{Router, middleware, routing::post};

use crate::{
    handlers::auth::{login, logout, refresh},
    middleware::auth::{validate_basic_auth, validate_refresh_token},
    state::AppState,
};

const AUTH_PATH: &str = "/auth";
const LOGIN_PATH: &str = "/login";
const LOGOUT_PATH: &str = "/logout";
const REFRESH_PATH: &str = "/refresh";

pub fn router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        AUTH_PATH,
        Router::new()
            .merge(basic_auth_router(state.clone()))
            .merge(refresh_token_auth_router(state.clone())),
    )
}

fn basic_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(LOGIN_PATH, post(login))
        .layer(middleware::from_fn_with_state(state, validate_basic_auth))
}

fn refresh_token_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(REFRESH_PATH, post(refresh))
        .route(LOGOUT_PATH, post(logout))
        .layer(middleware::from_fn_with_state(
            state,
            validate_refresh_token,
        ))
}
