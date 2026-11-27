use axum::extract::State;

use crate::{
    error::AppResult,
    payloads::{Many, unit::UnitResponse},
    responses::Get,
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_units(State(state): State<AppState>) -> AppResult<Get<Many<UnitResponse>>> {
    let units = state.unit_service.find_units().await?;
    Ok(units.into())
}
