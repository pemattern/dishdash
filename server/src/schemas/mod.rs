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

pub struct GetOneResponse<S>
where
    S: Serialize,
{
    schema: S,
}

impl<S, M> From<M> for GetOneResponse<S>
where
    S: Serialize + From<M>,
    M: Model,
{
    fn from(model: M) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<S> IntoResponse for GetOneResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schema)).into_response()
    }
}

pub struct GetManyResponse<S>
where
    S: Serialize,
{
    schemas: Vec<S>,
}

impl<S, M> From<Vec<M>> for GetManyResponse<S>
where
    S: Serialize + From<M>,
    M: Model,
{
    fn from(models: Vec<M>) -> Self {
        let schemas = models.into_iter().map(|model| From::from(model)).collect();
        Self { schemas }
    }
}

impl<S> IntoResponse for GetManyResponse<S>
where
    S: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schemas)).into_response()
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
