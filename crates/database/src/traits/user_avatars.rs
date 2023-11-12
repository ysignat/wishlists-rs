use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use thiserror::Error;
use uuid::Uuid;

pub type Key = Uuid;
pub type Value = ByteStream;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error")]
    Unknown,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn get_item_picture(&self, key: Key) -> Result<Value, Error>;
    async fn put_item_picture(&self, key: Key, value: Value) -> Result<(), Error>;
    async fn delete_item_picture(&self, key: Key) -> Result<(), Error>;
}
