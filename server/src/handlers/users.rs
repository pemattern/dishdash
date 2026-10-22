use crate::{
    auth::token::Claims,
    error::{AppResult, ClientError},
    schemas::user::{
        DeleteUserResponse, GetUserResponse, PatchUserRequest, PatchUserResponse, PostUserRequest,
        PostUserResponse,
    },
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<GetUserResponse> {
    let user = state
        .postgres_service
        .find_user_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<GetUserResponse> {
    let id = Uuid::parse_str(&claims.sub).unwrap();
    let user = state
        .postgres_service
        .find_user_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<PostUserRequest>,
) -> AppResult<PostUserResponse> {
    let user = state.postgres_service.create_user(user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user): Json<PatchUserRequest>,
) -> AppResult<PatchUserResponse> {
    let user = state.postgres_service.update_user(id, user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<DeleteUserResponse> {
    let user = state.postgres_service.delete_user(id).await?;
    Ok(user.into())
}
