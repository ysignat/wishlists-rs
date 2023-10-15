#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use aws_sdk_s3::Client;
use sea_orm::DatabaseConnection;
use thiserror::Error;

mod implementations;
pub mod interfaces;

#[derive(Debug, Error)]
enum Error {
    #[error("Unknown error")]
    Unknown,
}

pub struct Repository {
    database_connection: DatabaseConnection,
    s3_client: Client,
}

impl Repository {
    #[must_use]
    fn new(database_connection: DatabaseConnection, s3_client: Client) -> Self {
        Self {
            database_connection,
            s3_client,
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
