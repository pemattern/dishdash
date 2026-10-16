use crate::{error::RepositoryResult, models::PhysicalQuantity};
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait PhysicalQuantityRepository: Send + Sync {
    async fn get_all(&self) -> RepositoryResult<Vec<PhysicalQuantity>>;
}

#[derive(Clone)]
pub struct PostgresPhysicalQuantityRepository {
    pool: PgPool,
}

impl PostgresPhysicalQuantityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PhysicalQuantityRepository for PostgresPhysicalQuantityRepository {
    async fn get_all(&self) -> RepositoryResult<Vec<PhysicalQuantity>> {
        let query = "
            SELECT * FROM physical_quantities;
        ";
        let physical_quantities = sqlx::query_as::<_, PhysicalQuantity>(query)
            .fetch_all(&self.pool)
            .await?;
        Ok(physical_quantities)
    }
}
