use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;

use super::traits::item_pictures::{Error, Key, RepositoryTrait, Value};
use crate::Repository;

#[async_trait]
impl RepositoryTrait for Repository {
    async fn get_item_picture(&self, key: Key) -> Result<Value, Error> {
        self.blob_storage_client
            .get_object()
            .key(key.to_string())
            .send()
            .await
            .map(|x| x.body)
            .or(Err(Error::Unknown))
    }

    async fn put_item_picture(&self, key: Key, value: ByteStream) -> Result<(), Error> {
        self.blob_storage_client
            .put_object()
            .key(key.to_string())
            .body(value)
            .send()
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }

    async fn delete_item_picture(&self, key: Key) -> Result<(), Error> {
        self.blob_storage_client
            .delete_object()
            .key(key.to_string())
            .send()
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }
}
