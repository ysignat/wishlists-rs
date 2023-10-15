use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;

use crate::{
    interfaces::item_pictures::{Error, Key, RepositoryTrait, Value},
    Repository,
};

#[async_trait]
impl RepositoryTrait for Repository {
    async fn get_item_picture(&self, key: Key) -> Result<Value, Error> {
        self.s3_client
            .get_object()
            .key(key.to_string())
            .send()
            .await
            .map(|x| x.body)
            .or(Err(Error::Unknown))
    }

    async fn put_item_picture(&self, key: Key, value: ByteStream) -> Result<(), Error> {
        self.s3_client
            .put_object()
            .key(key.to_string())
            .body(value)
            .send()
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }

    async fn delete_item_picture(&self, key: Key) -> Result<(), Error> {
        self.s3_client
            .delete_object()
            .key(key.to_string())
            .send()
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }
}
