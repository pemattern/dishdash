use chrono::{DateTime, Utc};
use lib::Payload;
use uuid::Uuid;

use crate::models::UserModel;

#[derive(Payload)]
struct UserPayload {
    #[payload(read_only)]
    pub id: Uuid,
    #[payload(read_only)]
    pub created_at: DateTime<Utc>,
    #[payload(read_only)]
    pub updated_at: DateTime<Utc>,
    pub email: String,
    #[payload(write_only)]
    pub password: String,
}

impl From<UserModel> for UserResponse {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            email: value.email,
        }
    }
}
