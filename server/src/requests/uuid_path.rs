use axum::{
    extract::{FromRequestParts, Path, rejection::PathRejection},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::error::AppError;

trait IsUuid {}
impl IsUuid for Uuid {}
impl IsUuid for (Uuid, Uuid) {}

pub struct UuidPath<T>(pub T);

impl<S, T> FromRequestParts<S> for UuidPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + IsUuid + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = Path::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| match e {
                PathRejection::FailedToDeserializePathParams(_error) => AppError::InvalidUuidPath,
                PathRejection::MissingPathParams(_error) => AppError::MissingUuidPath,
                _ => unreachable!(),
            })?;
        Ok(Self(path.0))
    }
}
