pub mod auth;
pub mod error;

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::GenerateSchemas;
use serde::Serialize;

use crate::error::{AppError, AppResult};

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        if error.is_syntax() || error.is_eof() {
            AppError::InvalidJsonSyntax
        } else if error.is_data() {
            AppError::InvalidData
        } else {
            AppError::Internal(Arc::new(error))
        }
    }
}
fn json<T: Serialize>(value: T) -> AppResult<String> {
    Ok(serde_json::to_string_pretty(&value)?)
}

fn get_into_response<S: Serialize>(schema: S) -> Response {
    (StatusCode::OK, json(schema)).into_response()
}

fn post_into_response<S: Serialize>(schema: S) -> Response {
    (StatusCode::CREATED, json(schema)).into_response()
}

fn patch_into_response<S: Serialize>(schema: S) -> Response {
    (StatusCode::OK, json(schema)).into_response()
}

fn delete_into_response() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

pub struct GetResponse<Schema>
where
    Schema: Serialize,
{
    schema: Schema,
}

impl<Schema, Model> From<Model> for GetResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(model: Model) -> Self {
        let schema = From::from(model);
        Self { schema }
    }
}

impl<Schema> IntoResponse for GetResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        get_into_response(self.schema)
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
        get_into_response(self.schemas)
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
        post_into_response(self.schema)
    }
}

pub struct PostManyResponse<Schema>
where
    Schema: Serialize,
{
    schemas: Vec<Schema>,
}

impl<Schema, Model> From<Vec<Model>> for PostManyResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(models: Vec<Model>) -> Self {
        let schemas = models.into_iter().map(|model| From::from(model)).collect();
        Self { schemas }
    }
}

impl<Schema> IntoResponse for PostManyResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        post_into_response(self.schemas)
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
        patch_into_response(self.schema)
    }
}
pub struct PatchManyResponse<Schema>
where
    Schema: Serialize,
{
    schemas: Vec<Schema>,
}

impl<Schema, Model> From<Vec<Model>> for PatchManyResponse<Schema>
where
    Schema: Serialize + From<Model>,
    Model: GenerateSchemas,
{
    fn from(models: Vec<Model>) -> Self {
        let schemas = models.into_iter().map(|model| From::from(model)).collect();
        Self { schemas }
    }
}

impl<Schema> IntoResponse for PatchManyResponse<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        patch_into_response(self.schemas)
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
        delete_into_response()
    }
}
