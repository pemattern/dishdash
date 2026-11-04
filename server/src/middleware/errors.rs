use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::error::{AppError, AppResult};

#[axum::debug_middleware]
pub async fn errors(request: Request, next: Next) -> AppResult<impl IntoResponse> {
    let response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        match app_error.as_ref() {
            AppError::Client(client_error) => {
                // TODO manipulate response, include "instance": "uri"
                tracing::warn!("{:#?}", client_error);
            }
            AppError::Service(service_error) => tracing::error!("{:#?}", service_error),
        }
    }
    Ok(response)
}
