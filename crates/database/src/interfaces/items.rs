use async_trait::async_trait;
use chrono::NaiveDateTime;
use thiserror::Error;
use uuid::Uuid;

use super::{item_pictures, users, wishlists};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub type Id = Uuid;
pub type Predicate = &'static str;

pub struct Payload {
    pub id: Id,
    pub wishlist_id: wishlists::Id,
    pub selected_by_id: Option<users::Id>,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub picture_id: Option<item_pictures::Key>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Response {
    pub id: Id,
    pub wishlist_id: wishlists::Id,
    pub selected_by_id: Option<users::Id>,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub picture_id: Option<item_pictures::Key>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn create_item(&self, payload: Payload) -> Result<Response, Error>;
    async fn get_item(&self, id: Id) -> Result<Option<Response>, Error>;
    async fn list_items(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error>;
    async fn update_item(&self, id: Id, payload: Payload) -> Result<Response, Error>;
    async fn delete_item(&self, id: Id) -> Result<(), Error>;
}
