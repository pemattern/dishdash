use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::responses::json;

#[derive(Default, Debug, Serialize)]
pub struct ErrorSchema {
    title: String,
    status: u16,
    detail: Option<String>,
    instance: Option<String>,
}

impl ErrorSchema {
    pub fn unauthorized() -> Self {
        Self {
            title: String::from("Unauthorized"),
            status: 401,
            ..Self::default()
        }
    }

    pub fn not_found() -> Self {
        Self {
            title: String::from("Not found"),
            status: 404,
            ..Self::default()
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            title: String::from("Method not allowed"),
            status: 405,
            ..Self::default()
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            title: String::from("Unexpected error"),
            status: 500,
            ..Self::default()
        }
    }
}

impl IntoResponse for ErrorSchema {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status).unwrap(), json(self)).into_response()
    }
}
