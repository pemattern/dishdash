use sqlx::PgExecutor;

use crate::{error::AppResult, models::unit::Unit};

pub struct Units;

impl Units {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<Unit>> {
        let units = sqlx::query_as!(Unit, "SELECT * FROM units;")
            .fetch_all(executor)
            .await?;
        Ok(units)
    }
}
