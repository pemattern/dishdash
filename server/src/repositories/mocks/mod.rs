use std::{any::Any, collections::HashMap};

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::Model,
    repositories::{Repository, SelectAll},
};

pub struct MockRepository {
    tables: HashMap<String, Box<dyn Any>>,
}

impl MockRepository {
    pub fn new() -> Self {
        let tables = HashMap::new();
        Self { tables }
    }

    fn get_table<M: Model + Clone + 'static>(&self) -> RepositoryResult<Vec<M>> {
        let Some(rows) = self.tables.get(M::TABLE_NAME) else {
            return Err(RepositoryError::Mock(format!(
                "no tables found in mock repository for table_name: '{}'",
                M::TABLE_NAME
            )));
        };
        let Some(models) = rows.downcast_ref::<Vec<M>>() else {
            return Err(RepositoryError::Mock(format!(
                "unable to downcast table entries for table_name: '{}'",
                M::TABLE_NAME
            )));
        };
        Ok(models.clone())
    }
}

impl Repository for MockRepository {}

impl<M> SelectAll<MockRepository> for M
where
    M: Model + 'static,
{
    async fn select_all(repository: MockRepository) -> RepositoryResult<Vec<Self>> {
        let rows = repository.get_table::<M>()?;
        Ok(rows)
    }
}
