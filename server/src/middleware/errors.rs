use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::error::{AppError, AppResult};

#[axum::debug_middleware]
pub async fn handle_errors_middleware(
    request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        log::error!("Critical server error: {:#?}", app_error);
    }
    Ok(response)
}
