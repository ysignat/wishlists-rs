#![warn(clippy::pedantic)]
use async_trait::async_trait;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PrimaryKeyTrait};
use structs::{items, users, wishlists};
use thiserror::Error;
use uuid::Uuid;

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

#[async_trait]
pub trait Repository {
    async fn create_item(
        &self,
        payload: items::CreatePayload,
    ) -> Result<entities::items::Model, DbErr>;

    async fn get_item(&self, id: Uuid) -> Result<Option<entities::items::Model>, DbErr>;

    async fn list_items(&self) -> Result<Vec<entities::items::Model>, DbErr>;

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::UpdatePayload,
    ) -> Result<entities::items::Model, DbErr>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_user(
        &self,
        payload: users::CreatePayload,
    ) -> Result<entities::users::Model, DbErr>;

    async fn get_user(&self, id: Uuid) -> Result<Option<entities::users::Model>, DbErr>;

    async fn list_users(&self) -> Result<Vec<entities::users::Model>, DbErr>;

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::UpdatePayload,
    ) -> Result<entities::users::Model, DbErr>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_wishlist(
        &self,
        payload: wishlists::CreatePayload,
    ) -> Result<entities::wishlists::Model, DbErr>;

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<entities::wishlists::Model>, DbErr>;

    async fn list_wishlists(&self) -> Result<Vec<entities::wishlists::Model>, DbErr>;

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::UpdatePayload,
    ) -> Result<entities::wishlists::Model, DbErr>;

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr>;

    async fn healthcheck(&self) -> Result<(), DbErr>;
}

pub struct DatabaseRepository {
    pub database_connection: DatabaseConnection,
}

#[async_trait]
impl Repository for DatabaseRepository {
    async fn create_item(
        &self,
        payload: items::CreatePayload,
    ) -> Result<entities::items::Model, DbErr> {
        items::Crud {
            database_connection: &self.database_connection,
        }
        .create(payload)
        .await
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<entities::items::Model>, DbErr> {
        items::Crud {
            database_connection: &self.database_connection,
        }
        .get(id)
        .await
    }

    async fn list_items(&self) -> Result<Vec<entities::items::Model>, DbErr> {
        items::Crud {
            database_connection: &self.database_connection,
        }
        .list()
        .await
    }

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::UpdatePayload,
    ) -> Result<entities::items::Model, DbErr> {
        items::Crud {
            database_connection: &self.database_connection,
        }
        .update(id, payload)
        .await
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr> {
        items::Crud {
            database_connection: &self.database_connection,
        }
        .delete(id)
        .await
    }

    async fn create_user(
        &self,
        payload: users::CreatePayload,
    ) -> Result<entities::users::Model, DbErr> {
        users::Crud {
            database_connection: &self.database_connection,
        }
        .create(payload)
        .await
    }

    async fn get_user(&self, id: Uuid) -> Result<Option<entities::users::Model>, DbErr> {
        users::Crud {
            database_connection: &self.database_connection,
        }
        .get(id)
        .await
    }

    async fn list_users(&self) -> Result<Vec<entities::users::Model>, DbErr> {
        users::Crud {
            database_connection: &self.database_connection,
        }
        .list()
        .await
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::UpdatePayload,
    ) -> Result<entities::users::Model, DbErr> {
        users::Crud {
            database_connection: &self.database_connection,
        }
        .update(id, payload)
        .await
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr> {
        users::Crud {
            database_connection: &self.database_connection,
        }
        .delete(id)
        .await
    }

    async fn create_wishlist(
        &self,
        payload: wishlists::CreatePayload,
    ) -> Result<entities::wishlists::Model, DbErr> {
        wishlists::Crud {
            database_connection: &self.database_connection,
        }
        .create(payload)
        .await
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<entities::wishlists::Model>, DbErr> {
        wishlists::Crud {
            database_connection: &self.database_connection,
        }
        .get(id)
        .await
    }

    async fn list_wishlists(&self) -> Result<Vec<entities::wishlists::Model>, DbErr> {
        wishlists::Crud {
            database_connection: &self.database_connection,
        }
        .list()
        .await
    }

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::UpdatePayload,
    ) -> Result<entities::wishlists::Model, DbErr> {
        wishlists::Crud {
            database_connection: &self.database_connection,
        }
        .update(id, payload)
        .await
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr> {
        wishlists::Crud {
            database_connection: &self.database_connection,
        }
        .delete(id)
        .await
    }

    async fn healthcheck(&self) -> Result<(), DbErr> {
        todo!()
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
