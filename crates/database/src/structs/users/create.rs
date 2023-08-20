use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}
