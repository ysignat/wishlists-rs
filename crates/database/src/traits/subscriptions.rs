use async_trait::async_trait;
use chrono::NaiveDateTime;
use thiserror::Error;
use uuid::Uuid;

use super::users;

pub type Id = Uuid;

pub struct Payload {
    pub id: Id,
    pub user_id: users::Id,
    pub subscriber_id: users::Id,
    pub created_at: NaiveDateTime,
}

pub struct Response {
    pub id: Id,
    pub user_id: users::Id,
    pub subscriber_id: users::Id,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn create_subscription(&self, payload: Payload) -> Result<Response, Error>;
    async fn delete_subscription(&self, id: Id) -> Result<(), Error>;
}
