use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{middleware::auth::REFRESH_TOKEN_COOKIE, responses::json};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    #[serde(skip)]
    pub refresh_token: String,
}

impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    #[serde(skip)]
    pub refresh_token: String,
}

impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (
                    header::SET_COOKIE,
                    &set_refresh_token_cookie(&self.refresh_token),
                ),
            ],
            json(self),
        )
            .into_response()
    }
}

impl IntoResponse for RefreshResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (
                    header::SET_COOKIE,
                    &set_refresh_token_cookie(&self.refresh_token),
                ),
            ],
            json(self),
        )
            .into_response()
    }
}

fn set_refresh_token_cookie(refresh_token: &str) -> String {
    format!(
        "{}={}; HttpOnly; Secure; SameSite=Strict",
        REFRESH_TOKEN_COOKIE, refresh_token
    )
}

fn reset_refresh_token_cookie() -> String {
    let mut cookie = set_refresh_token_cookie("");
    cookie.push_str("; MaxAge=0");
    cookie
}

#[derive(Serialize)]
pub struct LogoutResponse;

impl IntoResponse for LogoutResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (header::SET_COOKIE, &reset_refresh_token_cookie()),
            ],
            json(self),
        )
            .into_response()
    }
}
