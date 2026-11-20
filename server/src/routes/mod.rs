mod auth;
mod data;
mod ingredients;
mod recipes;
mod users;

use std::time::Duration;

use axum::{
    Router,
    http::{Request, Response, StatusCode},
    middleware,
};
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::{DefaultOnFailure, TraceLayer},
};
use tracing::{Level, Span, info, info_span};

use crate::{
    error::{AppError, AppResult},
    middleware::{auth::validate_access_token_middleware, errors::handle_errors_middleware},
    routes::data::data_router,
    state::AppState,
};

const API_PATH: &str = "/api";
const EMPTY_PATH: &str = "/";

const STATIC_ASSETS_PATH: &str = "web/build";
const INDEX_HTML: &str = "/index.html";

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(api_router(state.clone()))
        .merge(data_router())
        .fallback_service(web_service())
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(handle_errors_middleware))
                .layer(
                    // wicked return types require inlining this
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            info_span!(
                                "request",
                                method = %request.method(),
                                path = %request.uri().path(),
                            )
                        })
                        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                            info!(
                                parent: span,
                                status = %response.status(),
                                latency = ?latency,
                            );
                        })
                        .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
                )
                .layer(compression_layer())
                .layer(timeout_layer()),
        )
        .with_state(state)
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
    // TODO: Timeout errors arent converted to AppErrors
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
        .layer(middleware::from_fn_with_state(
            state,
            validate_access_token_middleware,
        ))
}

fn web_service() -> ServeDir<ServeFile> {
    // TODO: middleware that sets cache headers
    let index_html_path = format!("{}{}", STATIC_ASSETS_PATH, INDEX_HTML);
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(index_html_path))
}

async fn method_not_allowed_fallback() -> AppResult<()> {
    Err(AppError::HttpMethod)
}
