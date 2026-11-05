use axum::{Extension, extract::State};
use uuid::Uuid;

use crate::{
    auth::token::Claims,
    error::AppResult,
    models::user::schemas::{PatchUserSchema, PostUserSchema, UserSchema},
    requests::{app_json::AppJson, app_path::AppPath},
    responses::{DeleteResponse, GetResponse, PatchResponse, PostResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
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
    AppJson(user): AppJson<PostUserSchema>,
) -> AppResult<PostResponse<UserSchema>> {
    let user = state.user_service.create_user(user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
    AppJson(user): AppJson<PatchUserSchema>,
) -> AppResult<PatchResponse<UserSchema>> {
    let user = state.user_service.update_user(id, user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    AppPath(id): AppPath<Uuid>,
) -> AppResult<DeleteResponse> {
    let user = state.user_service.delete_user(id).await?;
    Ok(user.into())
}
