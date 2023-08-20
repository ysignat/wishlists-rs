#![warn(clippy::pedantic)]
use errors::DataError;
use migrations::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

pub mod connection;
pub mod errors;
pub mod repository;
pub mod repository_trait;
pub mod structs;

/// # Errors
///
/// Will return `DataError` if migration attempt is unsuccessful
pub async fn migrate(connection: &DatabaseConnection) -> Result<(), DataError> {
    Migrator::up(connection, None)
        .await
        .or(Err(DataError::Unknown))
}
