use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::{
    error::{AppError, AppResult, BadRequestError, ClientError, ServiceError},
    responses::error::ErrorSchema,
};

// TODO: HANDLE no Content-Type: application/json
// TODO: HANDLE failed to deserialize

#[axum::debug_middleware]
pub async fn errors(request: Request, next: Next) -> AppResult<impl IntoResponse> {
    let uri = request.uri().clone();
    let mut response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        response = handle_app_error(app_error).instance(uri).into_response();
    }
    Ok(response)
}

fn handle_app_error(app_error: &AppError) -> ErrorSchema {
    match app_error {
        AppError::Client(client_error) => {
            tracing::warn!("{:#?}", client_error);
            handle_client_error(client_error)
        }
        AppError::Service(service_error) => {
            tracing::error!("{:#?}", service_error);
            handle_service_error(service_error)
        }
    }
}

fn handle_client_error(client_error: &ClientError) -> ErrorSchema {
    match client_error {
        ClientError::BadRequest(bad_request) => handle_bad_request_error(bad_request),
        ClientError::MethodNotAllowed => ErrorSchema::method_not_allowed(),
        ClientError::NotFound => ErrorSchema::not_found(),
        ClientError::Unauthorized => ErrorSchema::unauthorized(),
    }
}

fn handle_bad_request_error(bad_request_error: &BadRequestError) -> ErrorSchema {
    match bad_request_error {
        BadRequestError::PathRejection(path_rejection) => {
            ErrorSchema::bad_request().detail(&path_rejection.body_text())
        }
    }
}

fn handle_service_error(_service_error: &ServiceError) -> ErrorSchema {
    ErrorSchema::internal_server_error()
}
