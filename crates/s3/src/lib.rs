#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use async_trait::async_trait;
use aws_sdk_s3::{primitives::ByteStream, Client};
use thiserror::Error;

mod connection;

#[async_trait]
trait RepositoryTrait {
    async fn create(&self, key: &str, value: ByteStream) -> Result<(), S3Error>;
    async fn get(&self, key: &str) -> Result<ByteStream, S3Error>;
    async fn list(&self) -> Result<Vec<String>, S3Error>;
    async fn delete(&self, key: &str) -> Result<(), S3Error>;
}

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("Unknown s3 error")]
    Unknown,
}

struct Repository {
    client: Client,
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create(&self, key: &str, value: ByteStream) -> Result<(), S3Error> {
        self.client
            .put_object()
            .key(key)
            .body(value)
            .send()
            .await
            .map(|_| ())
            .or(Err(S3Error::Unknown))
    }

    async fn get(&self, key: &str) -> Result<ByteStream, S3Error> {
        self.client
            .get_object()
            .key(key)
            .send()
            .await
            .map(|x| x.body)
            .or(Err(S3Error::Unknown))
    }

    async fn list(&self) -> Result<Vec<String>, S3Error> {
        self.client
            .list_objects_v2()
            .send()
            .await
            .map(|x| {
                x.contents
                    .unwrap_or(Vec::new())
                    .iter()
                    .map(|y| y.key().unwrap().to_owned())
                    .collect()
            })
            .or(Err(S3Error::Unknown))
    }

    async fn delete(&self, key: &str) -> Result<(), S3Error> {
        self.client
            .delete_object()
            .key(key)
            .send()
            .await
            .map(|_| ())
            .or(Err(S3Error::Unknown))
    }
}

impl Repository {
    pub fn new(client: Client) -> Self {
        Repository { client }
    }
}
