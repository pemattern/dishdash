use std::sync::Arc;

use axum::{
    extract::Request,
    http::Uri,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    error::{AppError, AppResult, ClientError, ServiceError},
    responses::error::ErrorSchema,
};

#[axum::debug_middleware]
pub async fn errors(request: Request, next: Next) -> AppResult<impl IntoResponse> {
    let uri = request.uri().clone();
    let mut response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        response = handle_error_response(app_error, uri);
    }
    Ok(response)
}

fn handle_error_response(app_error: &AppError, uri: Uri) -> Response {
    match app_error {
        AppError::Client(client_error) => {
            tracing::warn!("{:#?}", client_error);
            client_error_response(client_error, uri)
        }
        AppError::Service(service_error) => {
            tracing::error!("{:#?}", service_error);
            service_error_response(uri)
        }
    }
}

fn client_error_response(client_error: &ClientError, uri: Uri) -> Response {
    let error_schema = match client_error {
        ClientError::MethodNotAllowed => ErrorSchema::method_not_allowed(),
        ClientError::NotFound => ErrorSchema::not_found(),
        ClientError::Unauthorized => ErrorSchema::unauthorized(),
    };
    error_schema.instance(uri).into_response()
}

fn service_error_response(uri: Uri) -> Response {
    ErrorSchema::internal_server_error()
        .instance(uri)
        .into_response()
}
