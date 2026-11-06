use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
    typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason},
};

use crate::error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(TypedHeader), rejection(AppError))]
pub struct AppTypedHeader<T>(pub T);

impl From<TypedHeaderRejection> for AppError {
    fn from(value: TypedHeaderRejection) -> Self {
        AppError::TypedHeaderRejection(value)
    }
}

pub struct BearerAuth(pub String);

impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|e| match e.reason() {
                    TypedHeaderRejectionReason::Missing => AppError::MissingAccessToken,
                    TypedHeaderRejectionReason::Error(error) => AppError::InvalidAccessToken,
                    _ => AppError::Internal,
                })?;

        Ok(Self {
            token: bearer.token().to_owned(),
        })
    }
}
