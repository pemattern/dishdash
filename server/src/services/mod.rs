use sqlx::{PgConnection, PgExecutor, PgPool};

use crate::error::{RepositoryResult, ServiceResult};

pub mod auth;
pub mod ingredient;
pub mod recipe;
pub mod user;

async fn insert_many_transaction<S, M, Q, F>(
    pool: &PgPool,
    schemas: Vec<S>,
    insert_query: Q,
) -> ServiceResult<Vec<M>>
where
    Q: Fn(&mut PgConnection, S) -> F,
    F: Future<Output = RepositoryResult<M>>,
{
    let mut tx = pool.begin().await?;
    let mut inserted = Vec::with_capacity(schemas.len());
    for schema in schemas {
        let row = insert_query(&mut *tx, schema).await?;
        inserted.push(row);
    }
    tx.commit().await?;
    Ok(inserted)
}
