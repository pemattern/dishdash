use axum::Router;
use tower_http::services::ServeDir;

use crate::{repositories::file_system::DATA_PATH, state::AppState};

pub fn data_router() -> Router<AppState> {
    let url = format!("/{}", DATA_PATH);
    Router::new().nest_service(&url, ServeDir::new(DATA_PATH))
}
