mod user;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::{BaseModel, Model},
    repositories::{DeleteById, Insert, Repository, SelectAll, SelectById, Update},
};

#[derive(Clone)]
pub struct MockRepository<M>
where
    M: Model,
{
    pub rows: Arc<Mutex<Vec<BaseModel<M>>>>,
}

impl<M> MockRepository<M>
where
    M: Model,
{
    pub fn new() -> Self {
        let rows = Arc::new(Mutex::new(Vec::new()));
        Self { rows }
    }
}

impl<M> Repository<M> for MockRepository<M> where M: Model {}

#[async_trait]
impl<M> Insert<M> for MockRepository<M>
where
    M: Model,
{
    async fn insert(&self, value: BaseModel<M>) -> RepositoryResult<BaseModel<M>> {
        let mut lock = self.rows.lock().unwrap();
        lock.push(value.clone());
        Ok(value)
    }
}

#[async_trait]
impl<M> SelectById<M> for MockRepository<M>
where
    M: Model,
{
    async fn select_by_id(&self, id: Uuid) -> RepositoryResult<Option<BaseModel<M>>> {
        let lock = self.rows.lock().unwrap();
        let model = lock.iter().find(|model| model.id == id);
        Ok(model.cloned())
    }
}

#[async_trait]
impl<M> SelectAll<M> for MockRepository<M>
where
    M: Model,
{
    async fn select_all(&self) -> RepositoryResult<Vec<BaseModel<M>>> {
        let lock = self.rows.lock().unwrap();
        Ok(lock.clone())
    }
}

#[async_trait]
impl<M> Update<M> for MockRepository<M>
where
    M: Model,
{
    async fn update(&self, value: BaseModel<M>) -> RepositoryResult<BaseModel<M>> {
        todo!()
    }
}

#[async_trait]
impl<M> DeleteById<M> for MockRepository<M>
where
    M: Model,
{
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<BaseModel<M>> {
        todo!()
    }
}
