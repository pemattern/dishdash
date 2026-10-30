pub mod auth;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::GenerateSchemas;
use serde::Serialize;

use crate::error::ServiceResult;

fn json<T: Serialize>(value: T) -> ServiceResult<String> {
    Ok(serde_json::to_string_pretty(&value)?)
}

pub struct GetOneResponse<Schema>
where
    Schema: Serialize,
{
    schema: Schema,
}

impl<Schema, Model> From<Model> for GetOneResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(model: Model) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<Schema> IntoResponse for GetOneResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schema)).into_response()
    }
}

pub struct GetManyResponse<Schema>
where
    Schema: Serialize,
{
    schemas: Vec<Schema>,
}

impl<Schema, Model> From<Vec<Model>> for GetManyResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(models: Vec<Model>) -> Self {
        let schemas = models.into_iter().map(|model| From::from(model)).collect();
        Self { schemas }
    }
}

impl<Schema> IntoResponse for GetManyResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schemas)).into_response()
    }
}

pub struct PostResponse<Schema>
where
    Schema: Serialize,
{
    schema: Schema,
}

impl<Schema, Model> From<Model> for PostResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(model: Model) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<Schema> IntoResponse for PostResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, json(self.schema)).into_response()
    }
}

pub struct PatchResponse<Schema>
where
    Schema: Serialize,
{
    schema: Schema,
}

impl<Schema, Model> From<Model> for PatchResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(model: Model) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<Schema> IntoResponse for PatchResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.schema)).into_response()
    }
}

pub struct DeleteResponse;

impl<Model> From<Model> for DeleteResponse
where
    Model: GenerateSchemas,
{
    fn from(_model: Model) -> Self {
        Self
    }
}

impl IntoResponse for DeleteResponse {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}
