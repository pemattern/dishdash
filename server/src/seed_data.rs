use std::{error::Error, path::PathBuf};

use sqlx::PgPool;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, BufReader},
};

const SEED_DATA_PATH: &str = "server/seed_data/";
const INGREDIENTS_CSV: &str = "ingredients.csv";
const INGREDIENT_TRANSLATIONS_CSV: &str = "ingredient_translations.csv";
const INGREDIENT_ALIASES_CSV: &str = "ingredient_aliases.csv";

pub async fn upsert_seed_data_tables(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    upsert_ingredient_seed_data(pool).await?;
    upsert_ingredient_translations_seed_data(pool).await?;
    upsert_ingredient_aliases_seed_data(pool).await?;
    Ok(())
}

async fn upsert_seed_data(
    pool: &PgPool,
    csv_file_name: &'static str,
    create_staging_table_query: &'static str,
    copy_csv_query: &'static str,
    upsert_table_query: &'static str,
) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(SEED_DATA_PATH);
    path.push(csv_file_name);
    let mut file = File::open(&path).await?;
    let checksum = checksum_file(&mut file).await?;
    file.rewind().await?;
    let previous_checksum: Option<Vec<u8>> = sqlx::query_scalar!(
        "SELECT checksum
        FROM seed_data_imports
        WHERE file_name = $1;",
        csv_file_name
    )
    .fetch_optional(pool)
    .await?;
    let needs_update = match previous_checksum {
        Some(previous_checksum) => checksum.as_bytes() != previous_checksum.as_slice(),
        None => true,
    };
    if !needs_update {
        return Ok(());
    }
    let mut tx = pool.begin().await?;
    sqlx::query(create_staging_table_query)
        .execute(&mut *tx)
        .await?;
    let mut copy = (&mut *tx).copy_in_raw(copy_csv_query).await?;
    copy.read_from(file).await?;
    copy.finish().await?;
    sqlx::query(upsert_table_query).execute(&mut *tx).await?;
    sqlx::query!(
        "INSERT INTO seed_data_imports (file_name, checksum)
        VALUES ($1, $2)
        ON CONFLICT (file_name)
        DO UPDATE SET
            checksum = EXCLUDED.checksum",
        csv_file_name,
        checksum.as_bytes()
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

async fn checksum_file(file: &mut File) -> std::io::Result<String> {
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

async fn upsert_ingredient_seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    upsert_seed_data(
        pool,
        INGREDIENTS_CSV,
        "CREATE TEMP TABLE ingredients_staging (
            code TEXT NOT NULL,
            ingredient_category_code TEXT NOT NULL
        ) ON COMMIT DROP;",
        "COPY ingredients_staging (code, ingredient_category_code)
        FROM STDIN
        WITH (FORMAT CSV, HEADER TRUE);",
        "INSERT INTO ingredients (code, ingredient_category_code)
        SELECT code, ingredient_category_code
        FROM ingredients_staging
        ON CONFLICT (code)
        DO UPDATE SET
            ingredient_category_code = EXCLUDED.ingredient_category_code;",
    )
    .await?;
    Ok(())
}

async fn upsert_ingredient_translations_seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    upsert_seed_data(
        pool,
        INGREDIENT_TRANSLATIONS_CSV,
        "CREATE TEMP TABLE ingredient_translations_staging (
            ingredient_code TEXT NOT NULL,
            locale_code TEXT NOT NULL,
            name TEXT NOT NULL,
            name_plural TEXT NOT NULL
        ) ON COMMIT DROP;",
        "COPY ingredient_translations_staging (ingredient_code, locale_code, name, name_plural)
            FROM STDIN
            WITH (FORMAT CSV, HEADER TRUE);",
        "INSERT INTO ingredient_translations (ingredient_code, locale_code, name, name_normalized, name_plural)
        SELECT ingredient_code, locale_code, name, search_normalize(name), name_plural
        FROM ingredient_translations_staging
        ON CONFLICT (ingredient_code, locale_code)
        DO UPDATE SET
            name = EXCLUDED.name,
            name_plural = EXCLUDED.name_plural,
            name_normalized = EXCLUDED.name_normalized
        WHERE ingredient_translations.name IS DISTINCT FROM EXCLUDED.name
            OR ingredient_translations.name_plural IS DISTINCT FROM EXCLUDED.name_plural
            OR ingredient_translations.name_normalized IS DISTINCT FROM EXCLUDED.name_normalized;"
    )
    .await?;
    Ok(())
}

async fn upsert_ingredient_aliases_seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    upsert_seed_data(
        pool,
        INGREDIENT_ALIASES_CSV,
        "CREATE TEMP TABLE ingredient_aliases_staging (
            ingredient_code TEXT NOT NULL,
            locale_code TEXT NOT NULL,
            name TEXT NOT NULL,
            name_plural TEXT NOT NULL
        ) ON COMMIT DROP;",
        "COPY ingredient_aliases_staging (ingredient_code, locale_code, name, name_plural)
            FROM STDIN
            WITH (FORMAT CSV, HEADER TRUE);",
        "INSERT INTO ingredient_aliases (ingredient_code, locale_code, name, name_normalized, name_plural)
        SELECT ingredient_code, locale_code, name, search_normalize(name), name_plural
        FROM ingredient_aliases_staging
        ON CONFLICT (ingredient_code, locale_code, name)
        DO UPDATE SET
            name_plural = EXCLUDED.name_plural,
            name_normalized = EXCLUDED.name_normalized
        WHERE ingredient_aliases.name_plural IS DISTINCT FROM EXCLUDED.name_plural
            OR ingredient_aliases.name_normalized IS DISTINCT FROM EXCLUDED.name_normalized;",
    )
    .await?;
    Ok(())
}
