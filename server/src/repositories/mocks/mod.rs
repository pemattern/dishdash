mod user;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::{BaseModel, Model},
    repositories::{DeleteById, Insert, Repository, SelectAll, SelectById},
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
    async fn insert(&self, value: M) -> RepositoryResult<BaseModel<M>> {
        let model = BaseModel::new(value);
        let mut lock = self.rows.lock().unwrap();
        lock.push(model.clone());
        Ok(model)
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
impl<M> DeleteById<M> for MockRepository<M>
where
    M: Model,
{
    async fn delete_by_id(&self, id: Uuid) -> RepositoryResult<BaseModel<M>> {
        let mut lock = self.rows.lock().unwrap();
        let rows: Vec<BaseModel<M>> = lock.extract_if(.., |row| row.id == id).collect();
        let row = assert_exactly_one(rows)?;
        Ok(row)
    }
}

fn assert_one_or_none<M: Model>(rows: Vec<BaseModel<M>>) -> RepositoryResult<Option<BaseModel<M>>> {
    if rows.len() == 0 {
        return Ok(None);
    }
    if rows.len() > 1 {
        return Err(RepositoryError::Mock("multiple rows found".into()));
    }
    Ok(Some(rows[0].clone()))
}

fn assert_exactly_one<M: Model>(rows: Vec<BaseModel<M>>) -> RepositoryResult<BaseModel<M>> {
    if rows.len() == 0 {
        return Err(RepositoryError::Mock("no rows found".into()));
    }
    if rows.len() > 1 {
        return Err(RepositoryError::Mock("multiple rows found".into()));
    }
    Ok(rows[0].clone())
}
