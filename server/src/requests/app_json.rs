use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
};

use crate::error::AppError;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        AppError::JsonRejection(value)
    }
}
