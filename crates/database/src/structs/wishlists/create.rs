use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}
