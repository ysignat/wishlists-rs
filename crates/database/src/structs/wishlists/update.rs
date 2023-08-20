use chrono::NaiveDateTime;
use entities::wishlists::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub name: String,
}

#[derive(Serialize)]
pub struct DatabaseResponse {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
