use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
};

use crate::error::AppError;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

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
