use async_trait::async_trait;
use chrono::NaiveDateTime;
use thiserror::Error;
use uuid::Uuid;

use super::{items, users};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub type Id = Uuid;
pub type Predicate = String;

pub struct Payload {
    pub id: Id,
    pub name: String,
    pub user_id: users::Id,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Response {
    pub id: Id,
    pub name: String,
    pub user_id: users::Id,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn create_wishlist(&self, payload: Payload) -> Result<Response, Error>;
    async fn get_wishlist(&self, id: Id) -> Result<Option<Response>, Error>;
    async fn list_wishlists(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error>;
    async fn update_wishlist(&self, id: Id, payload: Payload) -> Result<Response, Error>;
    async fn delete_wishlist(&self, id: Id) -> Result<(), Error>;

    async fn list_wishlist_items(
        &self,
        id: Id,
        predicate: Option<items::Predicate>,
    ) -> Result<Vec<items::Response>, Error>;
}
