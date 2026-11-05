use axum::{
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::responses::json;

#[derive(Default, Debug, Serialize)]
pub struct ErrorSchema {
    title: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
}

impl ErrorSchema {
    pub fn detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn instance(mut self, instance: Uri) -> Self {
        self.instance = Some(instance.to_string());
        self
    }

    pub fn get_status(&self) -> u16 {
        self.status
    }

    pub fn bad_request() -> Self {
        Self::from(StatusCode::BAD_REQUEST)
    }

    pub fn unauthorized() -> Self {
        Self::from(StatusCode::UNAUTHORIZED)
    }

    pub fn forbidden() -> Self {
        Self::from(StatusCode::FORBIDDEN)
    }

    pub fn not_found() -> Self {
        Self::from(StatusCode::NOT_FOUND)
    }

    pub fn method_not_allowed() -> Self {
        Self::from(StatusCode::METHOD_NOT_ALLOWED)
    }

    pub fn request_timeout() -> Self {
        Self::from(StatusCode::REQUEST_TIMEOUT)
    }

    pub fn conflict() -> Self {
        Self::from(StatusCode::CONFLICT)
    }

    pub fn unsupported_media_type() -> Self {
        Self::from(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    pub fn unprocessable_content() -> Self {
        Self::from(StatusCode::UNPROCESSABLE_ENTITY)
    }

    pub fn internal_server_error() -> Self {
        Self::from(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn service_unavailable() -> Self {
        Self::from(StatusCode::SERVICE_UNAVAILABLE)
    }
}

impl IntoResponse for ErrorSchema {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status).unwrap(), json(self)).into_response()
    }
}

impl From<StatusCode> for ErrorSchema {
    fn from(status_code: StatusCode) -> Self {
        Self {
            title: status_code.canonical_reason().unwrap().into(),
            status: status_code.as_u16(),
            ..Default::default()
        }
    }
}
