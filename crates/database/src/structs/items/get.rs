use chrono::NaiveDateTime;
use entities::items::Model;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DatabaseResponse {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub selected_by_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
