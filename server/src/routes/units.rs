use axum::{Router, routing::get};

use crate::{handlers::units::get_units, routes::EMPTY_PATH, state::AppState};

const UNITS_PATH: &str = "/units";

pub fn router() -> Router<AppState> {
    Router::new().nest(UNITS_PATH, Router::new().route(EMPTY_PATH, get(get_units)))
}
