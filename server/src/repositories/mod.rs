use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use sqlx::{AssertSqlSafe, FromRow, PgPool, postgres::PgRow};
use uuid::Uuid;

use crate::{
    error::RepositoryResult,
    models::{BaseModel, Model},
};

#[cfg(test)]
pub mod mocks;

pub mod ingredient;
pub mod physical_quantities;
pub mod recipe;
pub mod refresh_token;
pub mod user;

pub trait Repository<M>: SelectAll<M> + SelectById<M> + DeleteById<M> + Send + Sync
where
    M: Model,
{
}

impl<M> Repository<M> for PostgresRepository<M> where M: Model + for<'a> FromRow<'a, PgRow> {}

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
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            _marker: PhantomData,
        })
    }
}

#[async_trait]
pub trait Insert<M>
where
    M: Model,
{
    async fn insert(&self, value: BaseModel<M>) -> RepositoryResult<BaseModel<M>>;
}

#[async_trait]
pub trait SelectById<M>
where
    M: Model,
{
    async fn select_by_id(&self, id: Uuid) -> RepositoryResult<Option<BaseModel<M>>>;
}

#[async_trait]
impl<M> SelectById<M> for PostgresRepository<M>
where
    M: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_by_id(&self, id: Uuid) -> RepositoryResult<Option<BaseModel<M>>> {
        let query = format!("SELECT * FROM {} WHERE id = $1;", M::TABLE_NAME);
        let row = sqlx::query_as::<_, BaseModel<M>>(AssertSqlSafe(query))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }
}

#[async_trait]
pub trait SelectAll<M>
where
    M: Model,
{
    async fn select_all(&self) -> RepositoryResult<Vec<BaseModel<M>>>;
}

#[async_trait]
impl<M> SelectAll<M> for PostgresRepository<M>
where
    M: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_all(&self) -> RepositoryResult<Vec<BaseModel<M>>> {
        let query = format!("SELECT * FROM {};", M::TABLE_NAME);
        let rows = sqlx::query_as::<_, BaseModel<M>>(AssertSqlSafe(query))
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}

#[async_trait]
pub trait Update<M>
where
    M: Model,
{
    async fn update(&self, value: BaseModel<M>) -> RepositoryResult<BaseModel<M>>;
}

#[async_trait]
pub trait DeleteById<M>
where
    M: Model,
{
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<BaseModel<M>>;
}

#[async_trait]
impl<M> DeleteById<M> for PostgresRepository<M>
where
    M: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<BaseModel<M>> {
        let query = format!("DELETE FROM {} WHERE id = $1 RETURNING *;", M::TABLE_NAME);
        let row = sqlx::query_as::<_, BaseModel<M>>(AssertSqlSafe(query))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }
}
