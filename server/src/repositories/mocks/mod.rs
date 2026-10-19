mod user;

use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::{BaseModel, Model},
    repositories::{DeleteById, Insert, Repository, SelectAll, SelectById, Update},
};
impl Repository for MockRepository {}

#[derive(Clone)]
pub struct MockRepository {
    pub tables: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
}

impl MockRepository {
    pub fn new() -> Self {
        let tables = Arc::new(Mutex::new(HashMap::new()));
        Self { tables }
    }

    pub fn get_rows<M: Model + 'static>(&self) -> RepositoryResult<Vec<BaseModel<M>>> {
        let lock = self.tables.lock().unwrap();
        let rows = lock
            .get(M::TABLE_NAME)
            .ok_or(RepositoryError::Mock(format!(
                "no tables found in mock repository for table: '{}'",
                M::TABLE_NAME
            )))?;
        let models = rows
            .downcast_ref::<Vec<BaseModel<M>>>()
            .ok_or(RepositoryError::Mock(format!(
                "unable to downcast table entries for table: '{}'",
                M::TABLE_NAME
            )))?;
        Ok(models.clone())
    }

    pub fn set_rows<M: Model + 'static>(&self, value: Vec<BaseModel<M>>) -> RepositoryResult<()> {
        let mut lock = self.tables.lock().unwrap();
        lock.insert(M::TABLE_NAME.to_string(), Box::new(value))
            .ok_or(RepositoryError::Mock(format!(
                "unable to insert new values for table: '{}'",
                M::TABLE_NAME
            )));
        Ok(())
    }
}

impl Repository for &MockRepository {}

impl<M> Insert<&MockRepository> for M
where
    M: Model + 'static,
{
    async fn insert(
        repository: &MockRepository,
        value: BaseModel<Self>,
    ) -> RepositoryResult<BaseModel<Self>> {
        let mut rows = repository.get_rows::<M>()?;
        rows.push(value.clone());
        repository.set_rows::<M>(rows)?;
        Ok(value)
    }
}

impl<M> SelectById<&MockRepository> for M
where
    M: Model + 'static,
{
    async fn select_by_id(
        repository: &MockRepository,
        id: Uuid,
    ) -> RepositoryResult<Option<BaseModel<Self>>> {
        let rows = repository.get_rows::<M>()?;
        let model = rows.into_iter().find(|model| model.id == id);
        Ok(model)
    }
}

impl<M> SelectAll<&MockRepository> for M
where
    M: Model + 'static,
{
    async fn select_all(repository: &MockRepository) -> RepositoryResult<Vec<BaseModel<Self>>> {
        let rows = repository.get_rows::<M>()?;
        Ok(rows)
    }
}

impl<M> Update<&MockRepository> for M
where
    M: Model + 'static,
{
    async fn update(
        repository: &MockRepository,
        value: BaseModel<Self>,
    ) -> RepositoryResult<BaseModel<Self>> {
        let rows = repository.get_rows::<M>()?;
        let model =
            rows.into_iter()
                .find(|model| model.id == value.id)
                .ok_or(RepositoryError::Mock(format!(
                    "unable to get row with id: '{}' from table: '{}'",
                    value.id,
                    M::TABLE_NAME,
                )))?;
        Ok(model)
    }
}

impl<M> DeleteById<&MockRepository> for M
where
    M: Model + 'static,
{
    async fn delete_by_id(
        repository: &MockRepository,
        id: Uuid,
    ) -> RepositoryResult<BaseModel<Self>> {
        let mut rows = repository.get_rows::<M>()?;
        let removed: Vec<BaseModel<M>> = rows.extract_if(.., |model| model.id == id).collect();
        if removed.len() == 0 {
            return Err(RepositoryError::Mock("removed 0 rows".into()));
        }
        if removed.len() > 1 {
            return Err(RepositoryError::Mock("remove more than 1 row".into()));
        }
        Ok(removed.first().unwrap().clone())
    }
}
