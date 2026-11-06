use std::sync::Arc;

use axum::{
    Extension,
    response::{IntoResponse, Response},
};
use lib::BINDINGS_PATH;
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export, export_to = BINDINGS_PATH)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    Internal(#[serde(skip)] Box<dyn std::error::Error + Send + Sync>),
    ResourceNotFound,
    HttpMethodNotAllowed,
    InvalidBasicAuth,
    MissingBasicAuth,
    InvalidAccessToken,
    MissingAccessToken,
    InvalidRefreshToken,
    MissingRefreshToken,
    ExpiredRefreshToken,
    InvalidUuidPath,
    MissingUuidPath,
    AlreadyExists,
    InvalidData,
    InvalidJsonSyntax,
    MissingJsonContentType,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Extension(Arc::new(self)).into_response()
    }
}
