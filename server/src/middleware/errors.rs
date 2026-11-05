use std::sync::Arc;

use axum::{
    extract::Request,
    http::{
        StatusCode,
        header::{self, ALLOW},
    },
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
        AppError::Jsonwebtoken(error) => handle_jsonwebtoken_error(error),
        AppError::SerdeJson(error) => handle_serde_json_error(error),
        AppError::JsonRejection(json_rejection) => todo!(),
        AppError::PathRejection(path_rejection) => todo!(),
        AppError::TypedHeaderRejection(error) => handle_typed_header_rejection(error),
        AppError::IncorrectPassword => {
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
        Error::Configuration(error) => todo!(),
        Error::InvalidArgument(_) => todo!(),
        Error::Database(error) => handle_sqlx_database_error(error),
        Error::Io(error) => todo!(),
        Error::Tls(error) => todo!(),
        Error::Protocol(_) => todo!(),
        Error::RowNotFound => ErrorSchema::not_found(),
        Error::TypeNotFound { type_name } => todo!(),
        Error::ColumnIndexOutOfBounds { index, len } => todo!(),
        Error::ColumnNotFound(_) => todo!(),
        Error::ColumnDecode { index, source } => todo!(),
        Error::Encode(error) => todo!(),
        Error::Decode(error) => todo!(),
        Error::AnyDriverError(error) => todo!(),
        Error::PoolTimedOut => todo!(),
        Error::PoolClosed => todo!(),
        Error::WorkerCrashed => todo!(),
        Error::Migrate(error) => todo!(),
        Error::InvalidSavePointStatement => todo!(),
        Error::BeginFailed => todo!(),
        _ => todo!(),
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

fn handle_jsonwebtoken_error(error: &jsonwebtoken::errors::Error) -> ErrorSchema {
    use jsonwebtoken::errors::ErrorKind;

    match error.kind() {
        ErrorKind::InvalidToken => ErrorSchema::unauthorized(),
        ErrorKind::InvalidSignature => ErrorSchema::unauthorized(),
        ErrorKind::InvalidEcdsaKey => todo!(),
        ErrorKind::InvalidEddsaKey => todo!(),
        ErrorKind::InvalidRsaKey(_) => todo!(),
        ErrorKind::RsaFailedSigning => todo!(),
        ErrorKind::Signing(_) => todo!(),
        ErrorKind::InvalidAlgorithmName => ErrorSchema::unauthorized(),
        ErrorKind::UnsupportedAlgorithm => ErrorSchema::unauthorized(),
        ErrorKind::InvalidKeyFormat => todo!(),
        ErrorKind::MissingRequiredClaim(_) => ErrorSchema::unauthorized(),
        ErrorKind::InvalidClaimFormat(_) => ErrorSchema::unauthorized(),
        ErrorKind::ExpiredSignature => ErrorSchema::unauthorized(),
        ErrorKind::InvalidIssuer => todo!(),
        ErrorKind::InvalidAudience => todo!(),
        ErrorKind::InvalidSubject => todo!(),
        ErrorKind::ImmatureSignature => todo!(),
        ErrorKind::InvalidAlgorithm => todo!(),
        ErrorKind::MissingAlgorithm => todo!(),
        ErrorKind::Base64(decode_error) => todo!(),
        ErrorKind::Json(error) => todo!(),
        ErrorKind::Utf8(_) => todo!(),
        ErrorKind::Provider(_) => todo!(),
        _ => todo!(),
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

fn handle_typed_header_rejection(error: &TypedHeaderRejection) -> ErrorSchema {
    match (error.name(), error.reason()) {
        (&header::AUTHORIZATION, TypedHeaderRejectionReason::Missing) => {
            ErrorSchema::unauthorized().detail("Authentication credentials are required")
        }
        (_, _) => ErrorSchema::bad_request(),
    }
}
