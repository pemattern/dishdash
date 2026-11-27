use crate::error::AppResult;

pub mod auth;
pub mod ingredient;
pub mod recipe;
pub mod unit;
pub mod user;

async fn map_option_async<T, U, F>(value: &Option<T>, map: F) -> AppResult<Option<U>>
where
    T: Clone,
    F: AsyncFn(T) -> AppResult<U>,
{
    match value {
        None => Ok(None),
        Some(value) => Ok(Some(map(value.clone()).await?)),
    }
}

async fn map_double_option_async<T, U, F>(
    value: &Option<Option<T>>,
    map: F,
) -> AppResult<Option<Option<U>>>
where
    T: Clone,
    F: AsyncFn(T) -> AppResult<U>,
{
    match value {
        None => Ok(None),
        Some(None) => Ok(Some(None)),
        Some(Some(value)) => Ok(Some(Some(map(value.clone()).await?))),
    }
}
