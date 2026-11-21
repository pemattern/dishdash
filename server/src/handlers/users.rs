use axum::{Extension, extract::State};
use uuid::Uuid;

use crate::{
    auth::token::Claims,
    error::AppResult,
    payloads::user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    requests::{app_path::AppPath, validated_json::ValidatedJson},
    responses::{Delete, Get, Patch, Post},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<Get<UserResponse>> {
    let user = state.user_service.find_user_by_id(id).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Get<UserResponse>> {
    let id = claims.sub;
    let user = state.user_service.find_user_by_id(id).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    ValidatedJson(user): ValidatedJson<CreateUserRequest>,
) -> AppResult<Post<UserResponse>> {
    let user = state.user_service.create_user(user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
    ValidatedJson(user): ValidatedJson<UpdateUserRequest>,
) -> AppResult<Patch<UserResponse>> {
    let user = state.user_service.update_user(id, user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<Delete> {
    let user = state.user_service.delete_user(id).await?;
    Ok(user.into())
}
