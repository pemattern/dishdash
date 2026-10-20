pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

use crate::{error::RepositoryResult, models::Model};
use sqlx::AssertSqlSafe;
use sqlx::PgPool;
use std::marker::PhantomData;
use uuid::Uuid;

impl<M> PostgresRepository<M>
where
    M: Model,
{
    pub async fn select_by_id(&self, id: Uuid) -> RepositoryResult<Option<M>> {
        let query = format!("SELECT * FROM {} WHERE id = $1;", M::TABLE_NAME);
        let row = sqlx::query_as::<_, M>(AssertSqlSafe(query))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn select_all(&self) -> RepositoryResult<Vec<M>> {
        let query = format!("SELECT * FROM {};", M::TABLE_NAME);
        let rows = sqlx::query_as::<_, M>(AssertSqlSafe(query))
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<M> {
        let query = format!("DELETE FROM {} WHERE id = $1 RETURNING *;", M::TABLE_NAME);
        let row = sqlx::query_as::<_, M>(AssertSqlSafe(query))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }
}

#[derive(Clone)]
pub struct PostgresRepository<M>
where
    M: Model,
{
    pool: PgPool,
    _marker: PhantomData<M>,
}

impl<M> PostgresRepository<M>
where
    M: Model,
{
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
}
