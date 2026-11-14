use axum::{
    extract::{FromRequestParts, Query, rejection::QueryRejection},
    http::request::Parts,
};
use serde::Deserialize;

use crate::error::AppError;

fn default_locale() -> String {
    "en".into()
}

fn default_limit() -> i64 {
    10
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

impl<S> FromRequestParts<S> for SearchQuery
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<Self>::from_request_parts(parts, state)
            .await
            .map_err(AppError::from)?;
        Ok(query.0)
    }
}

impl From<QueryRejection> for AppError {
    fn from(error: QueryRejection) -> Self {
        match error {
            QueryRejection::FailedToDeserializeQueryString(_error) => {
                AppError::InvalidQueryParameter
            }
            _ => unreachable!(),
        }
    }
}
