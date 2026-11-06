use std::sync::Arc;

use axum::{
    extract::{
        Request,
        rejection::{JsonRejection, PathRejection},
    },
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};
use lib::BINDINGS_PATH;
use serde::Serialize;
use sqlx::error::DatabaseError;

use crate::{
    error::{AppError, AppResult},
    responses::error::ErrorSchema,
};

#[derive(Debug, Default, Serialize, ts_rs::TS)]
#[ts(export, export_to = BINDINGS_PATH)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    #[default]
    Internal,
    InvalidCredentials,
    MissingAuthorization,
    InvalidAccessToken,
    MissingAccessToken,
    InvalidRefreshToken,
    ExpiredRefreshToken,
    MissingRefreshToken,
    NotFound,
    HttpMethodNotAllowed,
    AlreadyExists,
    InvalidData,
    InvalidJsonSyntax,
    MissingJsonContentType,
    InvalidUuidPath,
    MissingUuidPath,
}

#[axum::debug_middleware]
pub async fn handle_errors_middleware(
    request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let _method = request.method().clone();
    let uri = request.uri().clone();
    let mut response = next.run(request).await;
    let extensions = response.extensions();
    if let Some(app_error) = extensions.get::<Arc<AppError>>() {
        let error_code = error_code_lookup(app_error);
        let error_schema = ErrorSchema::from(error_code);
        if error_schema.get_status() >= 500 {
            log::error!("Critical server error: {:#?}", app_error);
        }
        response = error_schema.instance(uri).into_response();
    }
    Ok(response)
}

fn error_code_lookup(app_error: &AppError) -> ErrorCode {
    match app_error {
        AppError::Sqlx(error) => sqlx_error_code_lookup(error),
        AppError::Argon2(error) => argon2_error_code_lookup(error),
        AppError::Argon2Phc(_error) => ErrorCode::Internal,
        AppError::Jsonwebtoken(_error) => ErrorCode::InvalidAccessToken,
        AppError::SerdeJson(error) => serde_json_error_code_lookup(error),
        AppError::JsonRejection(error) => json_rejection_error_code_lookup(error),
        AppError::PathRejection(error) => path_rejection_error_code_lookup(error),
        AppError::TypedHeaderRejection(error) => handle_typed_header_rejection(error),
        AppError::InvalidCredentials => ErrorCode::InvalidCredentials,
        AppError::InvalidRefreshToken => ErrorCode::InvalidRefreshToken,
        AppError::MissingRefreshToken => ErrorCode::MissingRefreshToken,
        AppError::HttpMethodNotAllowed => ErrorCode::HttpMethodNotAllowed,
        AppError::ExpiredRefreshToken => ErrorCode::ExpiredRefreshToken,
    }
}

fn sqlx_error_code_lookup(error: &sqlx::Error) -> ErrorCode {
    use sqlx::Error;
    match error {
        Error::Database(error) => handle_sqlx_database_error(error),
        Error::RowNotFound => ErrorCode::NotFound,
        _ => ErrorCode::Internal,
    }
}

fn handle_sqlx_database_error(error: &Box<dyn DatabaseError + 'static>) -> ErrorCode {
    use sqlx::error::ErrorKind;
    match error.kind() {
        ErrorKind::UniqueViolation => ErrorCode::AlreadyExists,
        ErrorKind::ForeignKeyViolation
        | ErrorKind::ExclusionViolation
        | ErrorKind::NotNullViolation
        | ErrorKind::CheckViolation => ErrorCode::InvalidData,
        _ => ErrorCode::Internal,
    }
}

fn argon2_error_code_lookup(error: &argon2::password_hash::Error) -> ErrorCode {
    match error {
        argon2::password_hash::Error::PasswordInvalid => ErrorCode::InvalidCredentials,
        _ => ErrorCode::Internal,
    }
}

fn serde_json_error_code_lookup(error: &serde_json::Error) -> ErrorCode {
    if error.is_syntax() || error.is_eof() {
        ErrorCode::InvalidJsonSyntax
    } else if error.is_data() {
        ErrorCode::InvalidData
    } else {
        ErrorCode::Internal
    }
}

fn json_rejection_error_code_lookup(error: &JsonRejection) -> ErrorCode {
    match error {
        JsonRejection::JsonDataError(_error) => ErrorCode::InvalidData,
        JsonRejection::JsonSyntaxError(_error) => ErrorCode::InvalidJsonSyntax,
        JsonRejection::MissingJsonContentType(_error) => ErrorCode::MissingJsonContentType,
        JsonRejection::BytesRejection(_error) => ErrorCode::Internal,
        _ => ErrorCode::Internal,
    }
}

fn path_rejection_error_code_lookup(error: &PathRejection) -> ErrorCode {
    match error {
        PathRejection::FailedToDeserializePathParams(_error) => ErrorCode::InvalidUuidPath,
        PathRejection::MissingPathParams(_error) => ErrorCode::MissingUuidPath,
        _ => todo!(),
    }
}

fn handle_typed_header_rejection(error: &TypedHeaderRejection) -> ErrorCode {
    match (error.name(), error.reason()) {
        (&header::AUTHORIZATION, TypedHeaderRejectionReason::Missing) => {
            ErrorCode::MissingAuthorization
        }
        (_, _) => ErrorCode::InvalidCredentials,
    }
}
