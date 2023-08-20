use chrono::NaiveDateTime;
use entities::users::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}

#[derive(Serialize)]
pub struct DatabaseResponse {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
