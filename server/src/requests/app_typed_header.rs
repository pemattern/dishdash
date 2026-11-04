use axum::extract::FromRequestParts;
use axum_extra::{TypedHeader, typed_header::TypedHeaderRejection};

use crate::error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(TypedHeader), rejection(AppError))]
pub struct AppTypedHeader<T>(pub T);

impl From<TypedHeaderRejection> for AppError {
    fn from(value: TypedHeaderRejection) -> Self {
        AppError::TypedHeaderRejection(value)
    }
}
