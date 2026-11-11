use std::{error::Error, path::PathBuf};

use sqlx::PgPool;
use tokio::fs::File;

const SEED_DATA_PATH: &str = "server/seed_data/";
const INGREDIENTS_CSV: &str = "ingredients.csv";
const INGREDIENT_NAMES_CSV: &str = "ingredient_names.csv";

pub async fn upsert_ingredient_seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "CREATE TEMP TABLE ingredients_staging (
        code TEXT NOT NULL,
        ingredient_category_code TEXT NOT NULL
    ) ON COMMIT DROP;"
    )
    .execute(&mut *tx)
    .await?;
    let mut path = PathBuf::from(SEED_DATA_PATH);
    path.push(INGREDIENTS_CSV);
    let file = File::open(&path).await?;
    let mut copy = (&mut *tx)
        .copy_in_raw(
            "COPY ingredients_staging (code, ingredient_category_code)
            FROM STDIN
            WITH (FORMAT CSV, HEADER TRUE);",
        )
        .await?;
    copy.read_from(file).await?;
    copy.finish().await?;
    sqlx::query(
        "INSERT INTO ingredients (code, ingredient_category_code)
        SELECT code, ingredient_category_code
        FROM ingredients_staging
        ON CONFLICT (code)
        DO UPDATE SET
            ingredient_category_code = EXCLUDED.ingredient_category_code;",
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn upsert_ingredient_names_seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "CREATE TEMP TABLE ingredient_names_staging (
            ingredient_code TEXT NOT NULL,
            locale_code TEXT NOT NULL,
            name TEXT NOT NULL
        ) ON COMMIT DROP;"
    )
    .execute(&mut *tx)
    .await?;
    let mut path = PathBuf::from(SEED_DATA_PATH);
    path.push(INGREDIENT_NAMES_CSV);
    let file = File::open(&path).await?;
    let mut copy = (&mut *tx)
        .copy_in_raw(
            "COPY ingredient_names_staging (ingredient_code, locale_code, name)
            FROM STDIN
            WITH (FORMAT CSV, HEADER TRUE);",
        )
        .await?;
    copy.read_from(file).await?;
    copy.finish().await?;
    sqlx::query(
        "INSERT INTO ingredient_names (ingredient_code, locale_code, name, name_normalized)
        SELECT ingredient_code, locale_code, name, search_normalize(name)
        FROM ingredient_names_staging
        ON CONFLICT (ingredient_code, locale_code, name_normalized)
        DO UPDATE SET
            name = EXCLUDED.name
        WHERE ingredient_names.name
            IS DISTINCT FROM EXCLUDED.name;",
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
