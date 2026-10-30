mod auth;
mod ingredients;
mod recipes;
mod users;

use std::time::Duration;

use axum::{Router, http::StatusCode, middleware};
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    classify::{ServerErrorsAsFailures, SharedClassifier},
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
    error::{AppResult, ClientError},
    middleware::{auth::validate_access_token, log::log},
    state::AppState,
};

const API_PATH: &str = "/api";
const ID_PATH: &str = "/{uuid}";
const EMPTY_PATH: &str = "/";

const STATIC_ASSETS_PATH: &str = "web/build";
const INDEX_HTML: &str = "/index.html";

pub fn app(state: AppState) -> Router {
    default_router_layers()
        .merge(api_router(state.clone()))
        .fallback_service(web_service())
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .with_state(state)
}

fn default_router_layers() -> Router<AppState> {
    Router::new().layer(
        ServiceBuilder::new()
            .layer(trace_layer())
            .layer(compression_layer())
            .layer(timeout_layer())
            .layer(middleware::from_fn(log)),
    )
}

fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

fn compression_layer() -> CompressionLayer {
    CompressionLayer::new()
        .quality(CompressionLevel::Default)
        .br(true)
        .no_gzip()
        .no_deflate()
        .no_zstd()
}

fn timeout_layer() -> TimeoutLayer {
    TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5))
}

fn api_router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        API_PATH,
        Router::new()
            .merge(auth::router(state.clone()))
            .merge(access_token_auth_router(state.clone())),
    )
}

fn access_token_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(ingredients::router())
        .merge(recipes::router())
        .merge(users::router())
        .layer(middleware::from_fn_with_state(state, validate_access_token))
}

fn web_service() -> ServeDir<ServeFile> {
    // TODO: middleware that sets cache headers
    let index_html_path = format!("{}{}", STATIC_ASSETS_PATH, INDEX_HTML);
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(index_html_path))
}

async fn method_not_allowed_fallback() -> AppResult<()> {
    Err(ClientError::MethodNotAllowed.into())
}
