use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nickname: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nickname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
