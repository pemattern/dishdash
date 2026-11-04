use std::sync::Arc;

use axum::{
    Extension,
    extract::rejection::{JsonRejection, PathRejection},
    response::{IntoResponse, Response},
};
use axum_extra::typed_header::TypedHeaderRejection;

pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    Sqlx(sqlx::Error),
    Argon2(argon2::password_hash::Error),
    Jsonwebtoken(jsonwebtoken::errors::Error),
    Uuid(uuid::Error),
    SerdeJson(serde_json::Error),
    JsonRejection(JsonRejection),
    PathRejection(PathRejection),
    TypedHeaderRejection(TypedHeaderRejection),
    IncorrectPassword,
    InvalidRefreshToken,
    NoRefreshTokenCookie,
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

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Jsonwebtoken(value)
    }
}

impl From<uuid::Error> for AppError {
    fn from(value: uuid::Error) -> Self {
        Self::Uuid(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}
