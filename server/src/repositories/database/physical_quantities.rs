use sqlx::PgExecutor;

use crate::{error::AppResult, models::PhysicalQuantityModel};

pub struct PhysicalQuantities;

impl PhysicalQuantities {
    pub async fn select_all<'a>(
        executor: impl PgExecutor<'a>,
    ) -> AppResult<Vec<PhysicalQuantityModel>> {
        let physical_quantities =
            sqlx::query_as!(PhysicalQuantityModel, "SELECT * FROM physical_quantities;")
                .fetch_all(executor)
                .await?;
        Ok(physical_quantities)
    }
}
