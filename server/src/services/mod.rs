use std::pin::Pin;

use sqlx::{PgConnection, PgExecutor, PgPool};

use crate::error::{RepositoryResult, ServiceResult};

pub mod auth;
pub mod ingredient;
pub mod recipe;
pub mod user;

type RepositoryFuture<'a, M> = Pin<Box<dyn Future<Output = RepositoryResult<M>> + Send + 'a>>;

async fn insert_many_transaction<S, M, Q>(
    pool: &PgPool,
    schemas: Vec<S>,
    mut insert_query: Q,
) -> ServiceResult<Vec<M>>
where
    Q: for<'a> AsyncFnMut(&'a mut PgConnection, S) -> RepositoryResult<M>,
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
