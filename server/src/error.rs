use std::sync::Arc;

use axum::{
    Extension,
    extract::rejection::{JsonRejection, PathRejection},
    response::{IntoResponse, Response},
};
use axum_extra::typed_header::TypedHeaderRejection;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    Argon2(argon2::password_hash::Error),
    Argon2Phc(argon2::password_hash::phc::Error),
    Jsonwebtoken(jsonwebtoken::errors::Error),
    SerdeJson(serde_json::Error),
    JsonRejection(JsonRejection),
    PathRejection(PathRejection),
    TypedHeaderRejection(TypedHeaderRejection),
    HttpMethodNotAllowed,
    InvalidCredentials,
    InvalidRefreshToken,
    MissingRefreshToken,
    ExpiredRefreshToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Extension(Arc::new(self)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Argon2(value)
    }
}

impl From<argon2::password_hash::phc::Error> for AppError {
    fn from(value: argon2::password_hash::phc::Error) -> Self {
        Self::Argon2Phc(value)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jsonwebtoken(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}
