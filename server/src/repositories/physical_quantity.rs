use crate::{error::RepositoryResult, models::physical_quantity::PhysicalQuantity};
use sqlx::PgExecutor;

pub struct PhysicalQuantities;

impl PhysicalQuantities {
    pub async fn get_all<'a>(
        executor: impl PgExecutor<'a>,
    ) -> RepositoryResult<Vec<PhysicalQuantity>> {
        let physical_quantities =
            sqlx::query_as!(PhysicalQuantity, "SELECT * FROM physical_quantities;")
                .fetch_all(executor)
                .await?;
        Ok(physical_quantities)
    }
}
