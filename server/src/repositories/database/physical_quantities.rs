use sqlx::PgExecutor;

use crate::{error::AppResult, models::physical_quantity::PhysicalQuantity};

pub struct PhysicalQuantities;

impl PhysicalQuantities {
    pub async fn get_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<PhysicalQuantity>> {
        let physical_quantities =
            sqlx::query_as!(PhysicalQuantity, "SELECT * FROM physical_quantities;")
                .fetch_all(executor)
                .await?;
        Ok(physical_quantities)
    }
}
