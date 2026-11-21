pub mod auth;
pub mod error;

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::Payload;
use serde::Serialize;

use crate::error::{AppError, AppResult};

fn json<T: Serialize>(value: T) -> AppResult<String> {
    Ok(serde_json::to_string_pretty(&value)?)
}

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

pub struct Get<Payload>
where
    Payload: Serialize,
{
    payload: Payload,
}

impl<Payload> From<Payload> for Get<Payload>
where
    Payload: Serialize,
{
    fn from(payload: Payload) -> Self {
        Self { payload }
    }
}

impl<Payload> IntoResponse for Get<Payload>
where
    Payload: Serialize,
{
    fn into_response(self) -> Response {
        get_into_response(self.payload)
    }
}

pub struct Post<Payload>
where
    Payload: Serialize,
{
    payload: Payload,
}

impl<Payload> From<Payload> for Post<Payload>
where
    Payload: Serialize,
{
    fn from(payload: Payload) -> Self {
        Self { payload }
    }
}

impl<Schema> IntoResponse for Post<Schema>
where
    Schema: Serialize,
{
    fn into_response(self) -> Response {
        post_into_response(self.payload)
    }
}

pub struct Patch<Payload>
where
    Payload: Serialize,
{
    payload: Payload,
}

impl<Payload> From<Payload> for Patch<Payload>
where
    Payload: Serialize,
{
    fn from(payload: Payload) -> Self {
        Self { payload }
    }
}

impl<Payload> IntoResponse for Patch<Payload>
where
    Payload: Serialize,
{
    fn into_response(self) -> Response {
        patch_into_response(self.payload)
    }
}

pub struct Delete;

impl<Payload> From<Payload> for Delete
where
    Payload: Serialize,
{
    fn from(_payload: Payload) -> Self {
        Self
    }
}

impl IntoResponse for Delete {
    fn into_response(self) -> Response {
        delete_into_response()
    }
}
