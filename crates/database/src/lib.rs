#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use migrations::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use thiserror::Error;

pub mod connection;
pub mod crud;
pub mod repository;

/// # Errors
///
/// Will return `DataError` if migration attempt is unsuccessful
pub async fn migrate(connection: &DatabaseConnection) -> Result<(), DataError> {
    Migrator::up(connection, None)
        .await
        .or(Err(DataError::Unknown))
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Unknown database error")]
    Unknown,
}
