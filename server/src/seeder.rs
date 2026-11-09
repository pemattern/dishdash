use std::sync::Arc;

use sqlx::PgPool;
use tokio::fs::File;

use crate::error::{AppError, AppResult};

const INGREDIENTS_CSV: &str = "ingredients.csv";

pub async fn upsert_ingredient_seed_data(pool: &PgPool) -> AppResult<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!("CREATE TEMP TABLE ingredients_staging (name TEXT) ON COMMIT DROP;")
        .execute(&mut *tx)
        .await?;
    let file = File::open(INGREDIENTS_CSV).await?;
    let mut copy = (&mut *tx)
        .copy_in_raw(
            "COPY ingredients_staging (name)
            FROM STDIN
            WITH (FORMAT CSV, HEADER TRUE);",
        )
        .await?;
    copy.read_from(file).await?;
    copy.finish().await?;
    sqlx::query(
        "INSERT INTO ingredients (name)
        SELECT name
        FROM ingredients_staging
        ON CONFLICT (name)
        DO NOTHING;",
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Internal(Arc::new(error))
    }
}
