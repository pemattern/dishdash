use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    extract::CookieJar,
    headers::{
        Authorization,
        authorization::{Basic, Bearer},
    },
};

use crate::{
    auth::token::get_valid_access_token_claims,
    error::{AppError, AppResult},
    requests::app_typed_header::AppTypedHeader,
    state::AppState,
};

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

#[axum::debug_middleware]
pub async fn validate_access_token(
    AppTypedHeader(authorization_bearer_header): AppTypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let access_token = authorization_bearer_header.token();
    let claims = get_valid_access_token_claims(access_token)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_refresh_token(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let Some(refresh_token_cookie) = jar.get(REFRESH_TOKEN_COOKIE) else {
        return Err(AppError::NoRefreshTokenCookie);
    };
    let refresh_token = state
        .auth_service
        .get_valid_refresh_token(refresh_token_cookie.value())
        .await?;
    request.extensions_mut().insert(refresh_token);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_basic_auth(
    State(state): State<AppState>,
    AppTypedHeader(authorization_basic_header): AppTypedHeader<Authorization<Basic>>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let user = state
        .auth_service
        .get_user_from_basic_auth(
            authorization_basic_header.username(),
            authorization_basic_header.password(),
        )
        .await
        .map_err(|_| AppError::InvalidCredentials)?;
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
