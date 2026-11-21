use sqlx::PgExecutor;

use crate::{error::AppResult, models::UnitSystemModel};

pub struct UnitSystems;

impl UnitSystems {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<UnitSystemModel>> {
        let unit_systems = sqlx::query_as!(UnitSystemModel, "SELECT * FROM unit_systems;")
            .fetch_all(executor)
            .await?;
        Ok(unit_systems)
    }
}
