use crate::{
    models::{BaseModel, User},
    schemas::json,
};
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct PostUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct PatchUserRequest {
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PatchUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub id: Uuid,
}

impl IntoResponse for GetUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self)).into_response()
    }
}

impl From<BaseModel<User>> for GetUserResponse {
    fn from(value: BaseModel<User>) -> Self {
        Self {
            id: value.id,
            email: value.model.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl IntoResponse for PostUserResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/{}", self.id))],
            json(self),
        )
            .into_response()
    }
}

impl From<BaseModel<User>> for PostUserResponse {
    fn from(value: BaseModel<User>) -> Self {
        Self {
            id: value.id,
            email: value.model.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl IntoResponse for PatchUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self)).into_response()
    }
}

impl From<BaseModel<User>> for PatchUserResponse {
    fn from(value: BaseModel<User>) -> Self {
        Self {
            id: value.id,
            email: value.model.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl IntoResponse for DeleteUserResponse {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

impl From<BaseModel<User>> for DeleteUserResponse {
    fn from(value: BaseModel<User>) -> Self {
        Self { id: value.id }
    }
}
