use sqlx::{AssertSqlSafe, FromRow, PgConnection, postgres::PgRow};
use uuid::Uuid;

use crate::{error::RepositoryResult, models::Model};

#[cfg(test)]
pub mod mocks;

pub mod ingredient;
pub mod physical_quantities;
pub mod recipe;
pub mod refresh_token;
pub mod user;

trait Create
where
    Self: Model + for<'a> sqlx::FromRow<'a, sqlx::postgres::PgRow> + Send + Unpin,
{
    async fn create(connection: &mut PgConnection, value: &Self) -> RepositoryResult<Self>;
}

trait GetAll
where
    Self: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn get_all(connection: &mut PgConnection) -> RepositoryResult<Vec<Self>> {
        let query = format!("SELECT * FROM {};", Self::TABLE_NAME);
        let rows = sqlx::query_as::<_, Self>(AssertSqlSafe(query))
            .fetch_all(connection)
            .await?;
        Ok(rows)
    }
}

trait FindById
where
    Self: Model + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    async fn find_by_id(connection: &mut PgConnection, id: Uuid) -> RepositoryResult<Option<Self>> {
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
