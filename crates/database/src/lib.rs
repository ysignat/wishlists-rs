#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
pub use aws_sdk_s3::{
    config::{
        Config as BlobStorageConfig,
        Credentials as BlobStorageCredentials,
        Region as BlobStorageRegion,
    },
    Client as BlobStorageClient,
};
pub use sea_orm::{ConnectOptions as DatabaseConnectOptions, Database, DatabaseConnection};
use thiserror::Error;

mod item_pictures;
mod items;
mod subscriptions;
pub mod traits;
mod user_avatars;
mod users;
mod wishlists;

#[derive(Debug, Error)]
enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub trait RepositoryTrait:
    traits::item_pictures::RepositoryTrait
    + traits::items::RepositoryTrait
    + traits::subscriptions::RepositoryTrait
    + traits::user_avatars::RepositoryTrait
    + traits::users::RepositoryTrait
    + traits::wishlists::RepositoryTrait
{
}

impl RepositoryTrait for Repository {}

impl Repository {
    #[must_use]
    pub fn new(
        database_connection: DatabaseConnection,
        blob_storage_client: BlobStorageClient,
    ) -> Self {
        Self {
            database_connection,
            blob_storage_client,
        }
    }

    // async fn healthcheck(&self) -> Result<(), Error> {
    //     self.database_connection
    //         .execute_unprepared("SELECT 1;")
    //         .await
    //         .map(|_| ())
    //         .or(Err(Error::Unknown))
    // }
}

pub struct Repository {
    database_connection: DatabaseConnection,
    blob_storage_client: BlobStorageClient,
}
