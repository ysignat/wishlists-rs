use serde::{Deserialize, Serialize};

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
    pub id: u64,
    pub name: String,
    pub surname: String,
    pub nickname: String,
    pub created_at: String,
    pub last_updated_at: String,
}
