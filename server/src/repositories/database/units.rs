use sqlx::PgExecutor;

use crate::requests::constants::{PhysicalQuantity, UnitSystem};
use crate::{error::AppResult, models::UnitModel};

pub struct Units;

impl Units {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<UnitModel>> {
        let units = sqlx::query_as!(
            UnitModel,
            r#"SELECT
                code,
                physical_quantity AS "physical_quantity: PhysicalQuantity",
                unit_system AS "unit_system: UnitSystem",
                symbol,
                si_factor
            FROM units;"#
        )
        .fetch_all(executor)
        .await?;
        Ok(units)
    }
}
