use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::{
    error::{AppError, AppResult},
    responses::error::ErrorSchema,
};

#[axum::debug_middleware]
pub async fn handle_errors(request: Request, next: Next) -> AppResult<impl IntoResponse> {
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
        AppError::Sqlx(error) => handle_sqlx_error(error),
        AppError::Argon2(error) => todo!(),
        AppError::Jsonwebtoken(error) => todo!(),
        AppError::Uuid(error) => todo!(),
        AppError::SerdeJson(error) => todo!(),
        AppError::JsonRejection(json_rejection) => todo!(),
        AppError::PathRejection(path_rejection) => todo!(),
        AppError::IncorrectPassword => todo!(),
        AppError::InvalidRefreshToken => todo!(),
        AppError::TypedHeaderRejection(typed_header_rejection) => todo!(),
        AppError::NoRefreshTokenCookie => todo!(),
    }
}

fn handle_sqlx_error(error: &sqlx::Error) -> ErrorSchema {
    match error {
        sqlx::Error::Configuration(error) => todo!(),
        sqlx::Error::InvalidArgument(_) => todo!(),
        sqlx::Error::Database(database_error) => todo!(),
        sqlx::Error::Io(error) => todo!(),
        sqlx::Error::Tls(error) => todo!(),
        sqlx::Error::Protocol(_) => todo!(),
        sqlx::Error::RowNotFound => todo!(),
        sqlx::Error::TypeNotFound { type_name } => todo!(),
        sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
        sqlx::Error::ColumnNotFound(_) => todo!(),
        sqlx::Error::ColumnDecode { index, source } => todo!(),
        sqlx::Error::Encode(error) => todo!(),
        sqlx::Error::Decode(error) => todo!(),
        sqlx::Error::AnyDriverError(error) => todo!(),
        sqlx::Error::PoolTimedOut => todo!(),
        sqlx::Error::PoolClosed => todo!(),
        sqlx::Error::WorkerCrashed => todo!(),
        sqlx::Error::Migrate(migrate_error) => todo!(),
        sqlx::Error::InvalidSavePointStatement => todo!(),
        sqlx::Error::BeginFailed => todo!(),
        _ => todo!(),
    }
}
