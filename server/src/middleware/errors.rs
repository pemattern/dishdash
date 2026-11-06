use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::{
    error::{AppError, AppResult},
    responses::error::ErrorSchema,
};

#[axum::debug_middleware]
pub async fn handle_errors_middleware(
    request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let uri = request.uri().clone();
    let mut response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        match app_error.as_ref() {
            AppError::Internal(error) => log::error!("Critical server error: {:#?}", error),
            _ => {}
        }
        let error_schema = ErrorSchema::from((**app_error).clone()).instance(uri);
        response = error_schema.into_response();
    }
    Ok(response)
}
