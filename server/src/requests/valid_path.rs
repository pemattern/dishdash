use axum::{
    extract::{FromRequestParts, Path, rejection::PathRejection},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::error::{AppError, BadRequestError, ClientError};

// TODO: Add uuid parsing for better errors

pub struct ValidPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
    Path<T>: FromRequestParts<S, Rejection = PathRejection>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<T>::from_request_parts(parts, state)
            .await
            .map(|Path(value)| ValidPath(value))
            .map_err(AppError::from)
    }
}

impl From<PathRejection> for AppError {
    fn from(value: PathRejection) -> Self {
        AppError::Client(ClientError::BadRequest(BadRequestError::PathRejection(
            value,
        )))
    }
}
