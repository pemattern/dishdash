use sqlx::{AssertSqlSafe, FromRow, PgPool, postgres::PgRow};
use uuid::Uuid;

use crate::{error::RepositoryResult, models::Model};

#[cfg(test)]
pub mod mocks;

pub mod ingredient;
pub mod physical_quantities;
pub mod recipe;
pub mod refresh_token;
pub mod user;

trait Repository {}

struct PostgresRepository {
    pool: PgPool,
}

impl Repository for PostgresRepository {}

trait Insert<R>
where
    Self: Model,
    R: Repository,
{
    async fn insert(repository: R, value: &Self) -> RepositoryResult<Self>;
}

trait SelectById<R>
where
    Self: Model,
    R: Repository,
{
    async fn select_by_id(repository: R, id: Uuid) -> RepositoryResult<Option<Self>>;
}

impl<M> SelectById<PostgresRepository> for M
where
    Self: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_by_id(
        repository: PostgresRepository,
        id: Uuid,
    ) -> RepositoryResult<Option<Self>> {
        let query = format!("SELECT * FROM {} WHERE id = $1;", Self::TABLE_NAME);
        let row = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .bind(id)
            .fetch_optional(&repository.pool)
            .await?;
        Ok(row)
    }
}

trait SelectAll<R>
where
    Self: Model,
    R: Repository,
{
    async fn select_all(persistance: R) -> RepositoryResult<Vec<Self>>;
}

impl<M> SelectAll<PostgresRepository> for M
where
    M: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn select_all(repository: PostgresRepository) -> RepositoryResult<Vec<Self>> {
        let query = format!("SELECT * FROM {};", Self::TABLE_NAME);
        let rows = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .fetch_all(&repository.pool)
            .await?;
        Ok(rows)
    }
}

trait Update<R>
where
    Self: Model + for<'a> FromRow<'a, PgRow>,
    R: Repository,
{
    async fn update(repository: R, value: &Self) -> RepositoryResult<Self>;
}

trait DeleteById<R>
where
    Self: Model,
    R: Repository,
{
    async fn delete_by_id(repository: R, id: Uuid) -> RepositoryResult<Self>;
}

impl<M> DeleteById<PostgresRepository> for M
where
    Self: Model + for<'a> FromRow<'a, PgRow>,
{
    async fn delete_by_id(repository: PostgresRepository, id: Uuid) -> RepositoryResult<Self> {
        let query = format!(
            "DELETE FROM {} WHERE id = $1 RETURNING *;",
            Self::TABLE_NAME
        );
        let row = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .bind(id)
            .fetch_one(&repository.pool)
            .await?;
        Ok(row)
    }
}
