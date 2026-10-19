use crate::{
    error::RepositoryResult,
    models::{BaseModel, User},
    repositories::{mocks::MockRepository, user::UserRepository},
};

impl UserRepository<MockRepository> for User {
    async fn select_by_email(
        repository: MockRepository,
        email: &str,
    ) -> RepositoryResult<Option<BaseModel<User>>> {
        let rows = repository.get_rows::<User>()?;
        let users: Vec<BaseModel<User>> = rows
            .into_iter()
            .filter(|user| user.model.email == email)
            .collect();
        if users.len() == 0 {
            return Ok(None);
        }
        if users.len() > 1 {
            return Err(crate::error::RepositoryError::Mock(format!(
                "more than one user found for email: '{}'",
                email
            )));
        }
        Ok(Some(users[0].clone()))
    }
}
