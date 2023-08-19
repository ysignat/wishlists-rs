use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub wishlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}
