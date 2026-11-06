use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::error::AppError;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(AppError::from)?;
        value.validate()?;
        Ok(Self(value))
    }
}

impl From<JsonRejection> for AppError {
    fn from(error: JsonRejection) -> Self {
        match error {
            JsonRejection::JsonDataError(_error) => AppError::InvalidData,
            JsonRejection::JsonSyntaxError(_error) => AppError::InvalidJsonSyntax,
            JsonRejection::MissingJsonContentType(_error) => AppError::MissingJsonContentType,
            _ => AppError::Internal(Arc::new(error)),
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(_errors: ValidationErrors) -> Self {
        AppError::InvalidData
    }
}
