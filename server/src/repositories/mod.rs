use std::{any::Any, collections::HashMap};

use sqlx::{AssertSqlSafe, FromRow, PgConnection, PgPool, postgres::PgRow};
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::Model,
};

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

struct MockRepository {
    tables: HashMap<String, Vec<Box<dyn Any>>>,
}

impl MockRepository {
    fn get_table(&self, table_name: &str) -> RepositoryResult<&Vec<Box<dyn Any>>> {
        let Some(rows) = self.tables.get(table_name) else {
            return Err(RepositoryError::Mock(format!(
                "no tables found in mock repository for table_name: '{}'",
                table_name
            )));
        };
        Ok(rows)
    }
}

impl Repository for PostgresRepository {}
impl Repository for MockRepository {}

trait Insert<R>
where
    Self: Model + for<'a> sqlx::FromRow<'a, sqlx::postgres::PgRow> + Send + Unpin,
    R: Repository,
{
    async fn insert(repository: R, value: &Self) -> RepositoryResult<Self>;
}

trait SelectAll<R>
where
    Self: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
    R: Repository,
{
    async fn select_all(persistance: R) -> RepositoryResult<Vec<Self>>;
}

impl<M> SelectAll<PostgresRepository> for M
where
    M: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn select_all(repository: PostgresRepository) -> RepositoryResult<Vec<Self>> {
        let query = format!("SELECT * FROM {};", Self::TABLE_NAME);
        let rows = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .fetch_all(&repository.pool)
            .await?;
        Ok(rows)
    }
}

impl<M> SelectAll<MockRepository> for M
where
    M: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn select_all(repository: MockRepository) -> RepositoryResult<Vec<Self>> {
        let table = repository.get_table(M::TABLE_NAME)?;

        let rows = table
            .into_iter()
            .map(|row| {
                row.downcast::<Self>()
                    .map(|boxed| *boxed.clone())
                    .expect("unable to downcast to model")
            })
            .collect();
        Ok(rows)
    }
}

trait SelectById
where
    Self: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn select_by_id(
        connection: &mut PgConnection,
        id: Uuid,
    ) -> RepositoryResult<Option<Self>> {
        let query = format!("SELECT * FROM {} WHERE id = $1;", Self::TABLE_NAME);
        let row = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .bind(id)
            .fetch_optional(connection)
            .await?;
        Ok(row)
    }
}

trait Put
where
    Self: Model + for<'a> sqlx::FromRow<'a, sqlx::postgres::PgRow> + Send + Unpin,
{
    async fn put(connection: &mut PgConnection, value: &Self) -> RepositoryResult<Self>;
}

trait DeleteById
where
    Self: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn delete_by_id(connection: &mut PgConnection, id: Uuid) -> RepositoryResult<Self> {
        let query = format!(
            "DELETE FROM {} WHERE id = $1 RETURNING *;",
            Self::TABLE_NAME
        );
        let row = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .bind(id)
            .fetch_one(connection)
            .await?;
        Ok(row)
    }
}
