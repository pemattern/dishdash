use sqlx::PgExecutor;

use crate::{error::AppResult, models::unit_system::UnitSystem};

pub struct UnitSystems;

impl UnitSystems {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<UnitSystem>> {
        let unit_systems = sqlx::query_as!(UnitSystem, "SELECT * FROM unit_systems;")
            .fetch_all(executor)
            .await?;
        Ok(unit_systems)
    }
}
