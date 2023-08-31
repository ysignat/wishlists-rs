#![warn(clippy::pedantic)]
use async_trait::async_trait;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PrimaryKeyTrait};
use thiserror::Error;

pub mod connection;
pub mod structs;

#[async_trait]
pub trait EntityCrud<T, U, V, W>
where
    T: EntityTrait,
    U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    fn get_database_connection(&self) -> &DatabaseConnection;

    async fn create(&self, payload: V) -> Result<T::Model, DbErr>;

    async fn get(&self, id: U) -> Result<Option<T::Model>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find_by_id(id).one(database_connection).await
    }

    async fn list(&self) -> Result<Vec<T::Model>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find().all(database_connection).await
    }

    async fn update(&self, id: U, payload: W) -> Result<T::Model, DbErr>;

    async fn delete(&self, id: U) -> Result<(), DbErr> {
        let database_connection = self.get_database_connection();
        T::delete_by_id(id)
            .exec(database_connection)
            .await
            .map(|_| ())
    }
}

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
