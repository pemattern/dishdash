use sqlx::PgExecutor;

use crate::requests::constants::{PhysicalQuantityCode, UnitCode, UnitSystemCode};
use crate::{error::AppResult, models::UnitModel};

pub struct Units;

impl Units {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<UnitModel>> {
        let units = sqlx::query_as!(
            UnitModel,
            r#"SELECT
                code AS "code: UnitCode",
                physical_quantity_code AS "physical_quantity_code: PhysicalQuantityCode",
                unit_system_code AS "unit_system_code: UnitSystemCode",
                symbol,
                si_factor
            FROM units;"#
        )
        .fetch_all(executor)
        .await?;
        Ok(units)
    }
}
