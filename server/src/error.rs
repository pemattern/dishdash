use std::sync::Arc;

use axum::{
    Extension,
    response::{IntoResponse, Response},
};
use lib::BINDINGS_PATH;
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, ts_rs::TS)]
#[ts(export, export_to = BINDINGS_PATH)]
#[serde(rename = "ErrorCode", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    Internal(#[serde(skip)] Arc<dyn std::error::Error + Send + Sync>),
    ResourceNotFound,
    HttpMethod,
    InvalidBasicAuth,
    MissingBasicAuth,
    InvalidAccessToken,
    MissingAccessToken,
    InvalidRefreshToken,
    MissingRefreshToken,
    ExpiredRefreshToken,
    MissingUuidPath,
    AlreadyExists,
    InvalidData,
    InvalidJsonSyntax,
    MissingJsonContentType,
    InvalidQueryParameter,
    UnsupportedImageFormat,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Extension(Arc::new(self)).into_response()
    }
}

impl From<tokio::io::Error> for AppError {
    fn from(error: tokio::io::Error) -> Self {
        AppError::Internal(Arc::new(error))
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(_error: base64::DecodeError) -> Self {
        dbg!(&_error);
        AppError::InvalidData
    }
}
