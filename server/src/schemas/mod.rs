pub mod auth;
pub mod ingredient;
pub mod user;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::Model;
use serde::Serialize;

use crate::error::ServiceResult;

fn json<T: Serialize>(value: T) -> ServiceResult<String> {
    Ok(serde_json::to_string_pretty(&value)?)
}

pub struct GetResponse<S>
where
    S: Serialize,
{
    schema: S,
}

impl<S, M> From<M> for GetResponse<S>
where
    S: Serialize + From<M>,
    M: Model,
{
    fn from(model: M) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<S> IntoResponse for GetResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schema)).into_response()
    }
}

pub struct PostResponse<S>
where
    S: Serialize,
{
    schema: S,
}

impl<S, M> From<M> for PostResponse<S>
where
    S: Serialize + From<M>,
    M: Model,
{
    fn from(model: M) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<S> IntoResponse for PostResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, json(self.schema)).into_response()
    }
}

pub struct PatchResponse<S>
where
    S: Serialize,
{
    schema: S,
}

impl<S, M> From<M> for PatchResponse<S>
where
    S: Serialize + From<M>,
    M: Model,
{
    fn from(model: M) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<S> IntoResponse for PatchResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schema)).into_response()
    }
}

pub struct DeleteResponse;

impl<M> From<M> for DeleteResponse
where
    M: Model,
{
    fn from(_model: M) -> Self {
        Self
    }
}

impl IntoResponse for DeleteResponse {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}
