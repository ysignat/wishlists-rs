use async_trait::async_trait;
use chrono::NaiveDateTime;
use thiserror::Error;
use uuid::Uuid;

use super::{user_avatars, wishlists};

pub type Id = Uuid;
pub type Predicate = String;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub struct Payload {
    pub id: Id,
    pub name: String,
    pub avatar_id: Option<user_avatars::Key>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Response {
    pub id: Id,
    pub name: String,
    pub avatar_id: Option<user_avatars::Key>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn create_user(&self, payload: Payload) -> Result<Response, Error>;
    async fn get_user(&self, id: Id) -> Result<Option<Response>, Error>;
    async fn list_users(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error>;
    async fn update_user(&self, id: Id, payload: Payload) -> Result<Response, Error>;
    async fn delete_user(&self, id: Id) -> Result<(), Error>;

    async fn list_user_wishlists(
        &self,
        id: Id,
        predicate: Option<wishlists::Predicate>,
    ) -> Result<Vec<wishlists::Response>, Error>;

    async fn list_user_subscribers(
        &self,
        id: Id,
        predicate: Option<Predicate>,
    ) -> Result<Vec<Response>, Error>;
    async fn list_user_subscriptions(
        &self,
        id: Id,
        predicate: Option<Predicate>,
    ) -> Result<Vec<Response>, Error>;
}
