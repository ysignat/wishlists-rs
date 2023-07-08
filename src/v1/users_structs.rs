use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub surname: String,
    pub nickname: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub nickname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
