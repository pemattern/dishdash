use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{
    auth::token::get_valid_access_token_claims,
    error::{AppError, AppResult},
    requests::auth::{BasicAuth, BearerAuth},
    state::AppState,
};

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

#[axum::debug_middleware]
pub async fn validate_access_token_middleware(
    BearerAuth(bearer): BearerAuth,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let access_token = bearer.token();
    let claims = get_valid_access_token_claims(access_token)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_refresh_token_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let Some(refresh_token_cookie) = jar.get(REFRESH_TOKEN_COOKIE) else {
        return Err(AppError::MissingRefreshToken);
    };
    let refresh_token = state
        .auth_service
        .get_valid_refresh_token(refresh_token_cookie.value())
        .await?;
    request.extensions_mut().insert(refresh_token);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_basic_auth_middleware(
    State(state): State<AppState>,
    BasicAuth(basic): BasicAuth,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let user = state
        .auth_service
        .get_user_from_basic_auth(basic.username(), basic.password())
        .await
        .map_err(|_| AppError::InvalidBasicAuth)?;
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
