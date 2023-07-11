use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateWishlist {
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateWishlist {
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Wishlist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct WishlistQueryParams {
    pub user_id: Uuid,
}
