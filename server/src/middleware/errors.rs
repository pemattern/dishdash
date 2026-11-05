use std::sync::Arc;

use axum::{
    extract::{
        Request,
        rejection::{JsonRejection, PathRejection},
    },
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};
use sqlx::error::DatabaseError;

use crate::{
    error::{AppError, AppResult},
    responses::error::ErrorSchema,
};

#[axum::debug_middleware]
pub async fn handle_errors(request: Request, next: Next) -> AppResult<impl IntoResponse> {
    let _method = request.method().clone();
    let uri = request.uri().clone();
    let mut response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        let error_schema = handle_app_error(app_error);
        if error_schema.get_status() >= 500 {
            log::error!("Critical server error: {:#?}", app_error);
        }
        response = error_schema.instance(uri).into_response();
    }
    Ok(response)
}

fn handle_app_error(app_error: &AppError) -> ErrorSchema {
    match app_error {
        AppError::Sqlx(error) => handle_sqlx_error(error),
        AppError::Argon2(error) => handle_argon2_error(error),
        AppError::Argon2Phc(_error) => ErrorSchema::internal_server_error(),
        AppError::Jsonwebtoken(_error) => ErrorSchema::unauthorized(),
        AppError::SerdeJson(error) => handle_serde_json_error(error),
        AppError::JsonRejection(error) => handle_json_rejection(error),
        AppError::PathRejection(error) => handle_path_rejection(error),
        AppError::TypedHeaderRejection(error) => handle_typed_header_rejection(error),
        AppError::InvalidCredentials => {
            ErrorSchema::unauthorized().detail("The provided credentials are invalid")
        }
        AppError::InvalidRefreshToken => {
            ErrorSchema::unauthorized().detail("The refresh token is invalid")
        }
        AppError::NoRefreshTokenCookie => {
            ErrorSchema::unauthorized().detail("The `refresh_token` cookie is missing")
        }
        AppError::HttpMethodNotAllowed => ErrorSchema::method_not_allowed(),
    }
}

fn handle_sqlx_error(error: &sqlx::Error) -> ErrorSchema {
    use sqlx::Error;

    match error {
        Error::Database(error) => handle_sqlx_database_error(error),
        Error::RowNotFound => ErrorSchema::not_found(),
        _ => ErrorSchema::internal_server_error(),
    }
}

fn handle_sqlx_database_error(error: &Box<dyn DatabaseError + 'static>) -> ErrorSchema {
    use sqlx::error::ErrorKind;

    match error.kind() {
        ErrorKind::UniqueViolation
        | ErrorKind::ForeignKeyViolation
        | ErrorKind::ExclusionViolation => ErrorSchema::from(StatusCode::CONFLICT),
        ErrorKind::NotNullViolation | ErrorKind::CheckViolation => {
            ErrorSchema::from(StatusCode::UNPROCESSABLE_ENTITY)
        }
        ErrorKind::Other => match error.code().as_deref() {
            Some("40001" | "40P01" | "08001" | "08006" | "57P01" | "53300") => {
                ErrorSchema::service_unavailable()
            }
            _ => ErrorSchema::internal_server_error(),
        },
        _ => ErrorSchema::from(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn handle_argon2_error(error: &argon2::password_hash::Error) -> ErrorSchema {
    match error {
        argon2::password_hash::Error::PasswordInvalid => ErrorSchema::unauthorized(),
        _ => ErrorSchema::internal_server_error(),
    }
}

fn handle_serde_json_error(error: &serde_json::Error) -> ErrorSchema {
    if error.is_syntax() || error.is_eof() {
        ErrorSchema::bad_request().detail("Invalid JSON request body")
    } else if error.is_data() {
        ErrorSchema::unprocessable_content().detail("Unable to process request body")
    } else {
        ErrorSchema::internal_server_error()
    }
}

fn handle_json_rejection(error: &JsonRejection) -> ErrorSchema {
    match error {
        JsonRejection::JsonDataError(_error) => {
            ErrorSchema::unprocessable_content().detail("Unable to process request body")
        }
        JsonRejection::JsonSyntaxError(_error) => {
            ErrorSchema::bad_request().detail("Invalid JSON request body")
        }
        JsonRejection::MissingJsonContentType(_error) => ErrorSchema::unsupported_media_type()
            .detail("Expected request with `Content-Type: application/json`"),
        JsonRejection::BytesRejection(_error) => ErrorSchema::internal_server_error(),
        _ => todo!(),
    }
}

fn handle_path_rejection(error: &PathRejection) -> ErrorSchema {
    match error {
        PathRejection::FailedToDeserializePathParams(_error) => {
            ErrorSchema::bad_request().detail("Invalid UUID in path")
        }
        PathRejection::MissingPathParams(_error) => {
            ErrorSchema::bad_request().detail("Missing path parameter")
        }
        _ => todo!(),
    }
}

fn handle_typed_header_rejection(error: &TypedHeaderRejection) -> ErrorSchema {
    match (error.name(), error.reason()) {
        (&header::AUTHORIZATION, TypedHeaderRejectionReason::Missing) => {
            ErrorSchema::unauthorized().detail("Authentication credentials are required")
        }
        (_, _) => ErrorSchema::bad_request(),
    }
}
