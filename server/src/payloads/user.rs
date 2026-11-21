use chrono::{DateTime, Utc};
use lib::Payload;
use uuid::Uuid;

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
