use crate::{
    error::RepositoryResult,
    models::{BaseModel, User},
    repositories::{PostgresRepository, Repository},
};

pub trait UserRepository<R>
where
    R: Repository,
{
    async fn select_by_email(
        repository: R,
        email: &str,
    ) -> RepositoryResult<Option<BaseModel<User>>>;
}

impl UserRepository<PostgresRepository> for User {
    async fn select_by_email(
        repository: PostgresRepository,
        email: &str,
    ) -> RepositoryResult<Option<BaseModel<User>>> {
        let query = "
            SELECT * FROM users
            WHERE email = $1;
        ";
        let user = sqlx::query_as::<_, BaseModel<User>>(query)
            .bind(email)
            .fetch_optional(&repository.pool)
            .await?;
        Ok(user)
    }
}
