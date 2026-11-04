use std::sync::Arc;

use axum::{
    Extension,
    extract::rejection::PathRejection,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;
pub type ServiceResult<T> = Result<T, ServiceError>;
pub type RepositoryResult<T> = Result<T, RepositoryError>;

pub enum AppError {
    Client(ClientError),
    Service(ServiceError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Extension(Arc::new(self)).into_response()
    }
}

impl From<ClientError> for AppError {
    fn from(value: ClientError) -> Self {
        Self::Client(value)
    }
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        Self::Service(value)
    }
}

#[derive(Debug)]
pub enum ClientError {
    BadRequest(BadRequestError),
    MethodNotAllowed,
    NotFound,
    Unauthorized,
}

#[derive(Debug)]
pub enum BadRequestError {
    PathRejection(PathRejection),
}

#[derive(Debug)]
pub enum ServiceError {
    Repository(RepositoryError),
    Sqlx(sqlx::Error),
    Hash(argon2::password_hash::Error),
    Token(jsonwebtoken::errors::Error),
    Uuid(uuid::Error),
    Serialization(serde_json::Error),
    Validation,
}

impl From<RepositoryError> for ServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value)
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Hash(value)
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Token(value)
    }
}

impl From<uuid::Error> for ServiceError {
    fn from(value: uuid::Error) -> Self {
        Self::Uuid(value)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}
