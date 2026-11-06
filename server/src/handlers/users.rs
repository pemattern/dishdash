use axum::{Extension, extract::State};
use uuid::Uuid;

use crate::{
    auth::token::Claims,
    error::AppResult,
    models::user::schemas::{PatchUserSchema, PostUserSchema, UserSchema},
    requests::{app_json::ValidatedJson, uuid_path::UuidPath},
    responses::{DeleteResponse, GetResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
) -> AppResult<GetResponse<UserSchema>> {
    let user = state.user_service.find_user_by_id(id).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<GetResponse<UserSchema>> {
    let id = claims.sub;
    let user = state.user_service.find_user_by_id(id).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    ValidatedJson(user): ValidatedJson<PostUserSchema>,
) -> AppResult<PostResponse<UserSchema>> {
    let user = state.user_service.create_user(user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
    ValidatedJson(user): ValidatedJson<PatchUserSchema>,
) -> AppResult<PatchResponse<UserSchema>> {
    let user = state.user_service.update_user(id, user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    UuidPath(id): UuidPath<Uuid>,
) -> AppResult<DeleteResponse> {
    let user = state.user_service.delete_user(id).await?;
    Ok(user.into())
}
