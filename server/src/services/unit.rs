use sqlx::PgPool;

use crate::{
    error::AppResult,
    payloads::{Many, unit::UnitResponse},
    repositories::database::Units,
};

#[derive(Clone)]
pub struct UnitService {
    pool: PgPool,
}

impl UnitService {
    pub fn new(pool: &PgPool) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn find_units(&self) -> AppResult<Many<UnitResponse>> {
        let units = Units::select_all(&self.pool).await?;
        Ok(units.into())
    }
}
