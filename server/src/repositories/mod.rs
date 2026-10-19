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

pub trait Repository {}

#[derive(Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository for PostgresRepository {}

pub trait Insert<M>
where
    Self: Repository,
    M: Model,
{
    async fn insert(&self, value: BaseModel<M>) -> RepositoryResult<BaseModel<M>>;
}

pub trait SelectById<M>
where
    Self: Repository,
    M: Model,
{
    async fn select_by_id(&self, id: Uuid) -> RepositoryResult<Option<BaseModel<M>>>;
}

impl<M> SelectById<PostgresRepository> for M
where
    Self: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_by_id(
        repository: PostgresRepository,
        id: Uuid,
    ) -> RepositoryResult<Option<BaseModel<Self>>> {
        let query = format!("SELECT * FROM {} WHERE id = $1;", Self::TABLE_NAME);
        let row = sqlx::query_as::<_, BaseModel<Self>>(AssertSqlSafe(query))
            .bind(id)
            .fetch_optional(&repository.pool)
            .await?;
        Ok(row)
    }
}

pub trait SelectAll<R>
where
    Self: Model,
    R: Repository,
{
    async fn select_all(persistance: R) -> RepositoryResult<Vec<BaseModel<Self>>>;
}

impl<M> SelectAll<PostgresRepository> for M
where
    M: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_all(repository: PostgresRepository) -> RepositoryResult<Vec<BaseModel<Self>>> {
        let query = format!("SELECT * FROM {};", Self::TABLE_NAME);
        let rows = sqlx::query_as::<_, BaseModel<Self>>(AssertSqlSafe(query))
            .fetch_all(&repository.pool)
            .await?;
        Ok(rows)
    }
}

pub trait Update<R>
where
    Self: Model,
    R: Repository,
{
    async fn update(repository: R, value: BaseModel<Self>) -> RepositoryResult<BaseModel<Self>>;
}

pub trait DeleteById<R>
where
    Self: Model,
    R: Repository,
{
    async fn delete_by_id(repository: R, id: Uuid) -> RepositoryResult<BaseModel<Self>>;
}

impl<M> DeleteById<PostgresRepository> for M
where
    Self: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn delete_by_id(
        repository: PostgresRepository,
        id: Uuid,
    ) -> RepositoryResult<BaseModel<Self>> {
        let query = format!(
            "DELETE FROM {} WHERE id = $1 RETURNING *;",
            Self::TABLE_NAME
        );
        let row = sqlx::query_as::<_, BaseModel<Self>>(AssertSqlSafe(query))
            .bind(id)
            .fetch_one(&repository.pool)
            .await?;
        Ok(row)
    }
}
