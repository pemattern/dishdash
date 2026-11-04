use axum::{
    Json,
    extract::{FromRequestParts, rejection::JsonRejection},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::error::AppError;

pub struct ValidJson<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
    Json<T>: FromRequestParts<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request_parts(parts, state)
            .await
            .map(|Json(value)| ValidJson(value))
            .map_err(AppError::from)
    }
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        match value {
            JsonRejection::JsonDataError(json_data_error) => todo!(),
            JsonRejection::JsonSyntaxError(json_syntax_error) => todo!(),
            JsonRejection::MissingJsonContentType(missing_json_content_type) => todo!(),
            JsonRejection::BytesRejection(bytes_rejection) => todo!(),
            _ => todo!(),
        }
    }
}
