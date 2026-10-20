use crate::{
    auth::token::generate_access_token,
    error::AppResult,
    models::{BaseModel, refresh_token::RefreshToken, user::User},
    schemas::token::{LoginResponse, LogoutResponse, RefreshResponse},
    state::AppState,
};
use axum::{Extension, extract::State};

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<BaseModel<User>>,
) -> AppResult<LoginResponse> {
    let access_token = generate_access_token(user.id)?;
    let refresh_token = state
        .auth_service
        .generate_refresh_token_from_login(user.id)
        .await?;
    Ok(LoginResponse::new(access_token, refresh_token))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/refresh")]
pub async fn refresh(
    State(state): State<AppState>,
    Extension(refresh_token): Extension<BaseModel<RefreshToken>>,
) -> AppResult<RefreshResponse> {
    let access_token = generate_access_token(refresh_token.model.user_id)?;
    let composite_refresh_token = state
        .auth_service
        .cycle_refresh_token(&refresh_token)
        .await?;
    Ok(RefreshResponse::new(access_token, composite_refresh_token))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/logout")]
pub async fn logout(
    State(state): State<AppState>,
    Extension(refresh_token): Extension<BaseModel<RefreshToken>>,
) -> AppResult<LogoutResponse> {
    let _ = state
        .auth_service
        .mark_refresh_token_as_used(&refresh_token)
        .await?;
    Ok(LogoutResponse)
}
