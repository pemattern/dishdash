use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::{Basic, Bearer},
    },
    typed_header::TypedHeaderRejectionReason,
};

use crate::error::AppError;

pub struct BasicAuth(pub Basic);

impl<S> FromRequestParts<S> for BasicAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(basic)) =
            TypedHeader::<Authorization<Basic>>::from_request_parts(parts, state)
                .await
                .map_err(|e| match e.reason() {
                    TypedHeaderRejectionReason::Missing => AppError::MissingBasicAuth,
                    TypedHeaderRejectionReason::Error(_error) => AppError::InvalidBasicAuth,
                    _ => unreachable!(),
                })?;
        Ok(Self(basic))
    }
}

pub struct BearerAuth(pub Bearer);

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
                    TypedHeaderRejectionReason::Error(_error) => AppError::InvalidAccessToken,
                    _ => unreachable!(),
                })?;
        Ok(Self(bearer))
    }
}
