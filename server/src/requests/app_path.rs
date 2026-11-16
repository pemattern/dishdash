use axum::extract::{FromRequestParts, Path, rejection::PathRejection};

use crate::error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(Path), rejection(AppError))]
pub struct AppPath<T>(pub T);

impl From<PathRejection> for AppError {
    fn from(error: PathRejection) -> Self {
        match error {
            PathRejection::FailedToDeserializePathParams(_error) => AppError::ResourceNotFound,
            PathRejection::MissingPathParams(_error) => AppError::ResourceNotFound,
            _ => unreachable!(),
        }
    }
}
