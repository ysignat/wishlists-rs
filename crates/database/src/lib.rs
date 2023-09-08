#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use async_trait::async_trait;
use crud::{CrudTrait, ItemsCrud, UsersCrud, WishlistsCrud};
pub use crud::{
    ItemsDatabaseCreatePayload,
    ItemsDatabaseResponse,
    ItemsDatabaseUpdatePayload,
    UsersDatabaseCreatePayload,
    UsersDatabaseResponse,
    UsersDatabaseUpdatePayload,
    WishlistsDatabaseCreatePayload,
    WishlistsDatabaseResponse,
    WishlistsDatabaseUpdatePayload,
};
pub use migrations::{Migrator, MigratorTrait};
pub use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm::{ConnectionTrait, DbErr};
use thiserror::Error;
use uuid::Uuid;

mod crud;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Unknown database error")]
    Unknown,
}

#[async_trait]
pub trait RepositoryTrait {
    async fn create_item(
        &self,
        payload: ItemsDatabaseCreatePayload,
    ) -> Result<ItemsDatabaseResponse, DbErr>;

    async fn get_item(&self, id: Uuid) -> Result<Option<ItemsDatabaseResponse>, DbErr>;

    async fn list_items(&self) -> Result<Vec<ItemsDatabaseResponse>, DbErr>;

    async fn update_item(
        &self,
        id: Uuid,
        payload: ItemsDatabaseUpdatePayload,
    ) -> Result<ItemsDatabaseResponse, DbErr>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_user(
        &self,
        payload: UsersDatabaseCreatePayload,
    ) -> Result<UsersDatabaseResponse, DbErr>;

    async fn get_user(&self, id: Uuid) -> Result<Option<UsersDatabaseResponse>, DbErr>;

    async fn list_users(&self) -> Result<Vec<UsersDatabaseResponse>, DbErr>;

    async fn update_user(
        &self,
        id: Uuid,
        payload: UsersDatabaseUpdatePayload,
    ) -> Result<UsersDatabaseResponse, DbErr>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_wishlist(
        &self,
        payload: WishlistsDatabaseCreatePayload,
    ) -> Result<WishlistsDatabaseResponse, DbErr>;

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<WishlistsDatabaseResponse>, DbErr>;

    async fn list_wishlists(&self) -> Result<Vec<WishlistsDatabaseResponse>, DbErr>;

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: WishlistsDatabaseUpdatePayload,
    ) -> Result<WishlistsDatabaseResponse, DbErr>;

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr>;

    async fn healthcheck(&self) -> Result<(), DbErr>;
}

pub struct Repository {
    database_connection: DatabaseConnection,
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_item(
        &self,
        payload: ItemsDatabaseCreatePayload,
    ) -> Result<ItemsDatabaseResponse, DbErr> {
        ItemsCrud::create(&self.database_connection, payload).await
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<ItemsDatabaseResponse>, DbErr> {
        ItemsCrud::get(&self.database_connection, id).await
    }

    async fn list_items(&self) -> Result<Vec<ItemsDatabaseResponse>, DbErr> {
        ItemsCrud::list(&self.database_connection).await
    }

    async fn update_item(
        &self,
        id: Uuid,
        payload: ItemsDatabaseUpdatePayload,
    ) -> Result<ItemsDatabaseResponse, DbErr> {
        ItemsCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr> {
        ItemsCrud::delete(&self.database_connection, id).await
    }

    async fn create_user(
        &self,
        payload: UsersDatabaseCreatePayload,
    ) -> Result<UsersDatabaseResponse, DbErr> {
        UsersCrud::create(&self.database_connection, payload).await
    }

    async fn get_user(&self, id: Uuid) -> Result<Option<UsersDatabaseResponse>, DbErr> {
        UsersCrud::get(&self.database_connection, id).await
    }

    async fn list_users(&self) -> Result<Vec<UsersDatabaseResponse>, DbErr> {
        UsersCrud::list(&self.database_connection).await
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: UsersDatabaseUpdatePayload,
    ) -> Result<UsersDatabaseResponse, DbErr> {
        UsersCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr> {
        UsersCrud::delete(&self.database_connection, id).await
    }

    async fn create_wishlist(
        &self,
        payload: WishlistsDatabaseCreatePayload,
    ) -> Result<WishlistsDatabaseResponse, DbErr> {
        WishlistsCrud::create(&self.database_connection, payload).await
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<WishlistsDatabaseResponse>, DbErr> {
        WishlistsCrud::get(&self.database_connection, id).await
    }

    async fn list_wishlists(&self) -> Result<Vec<WishlistsDatabaseResponse>, DbErr> {
        WishlistsCrud::list(&self.database_connection).await
    }

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: WishlistsDatabaseUpdatePayload,
    ) -> Result<WishlistsDatabaseResponse, DbErr> {
        WishlistsCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr> {
        WishlistsCrud::delete(&self.database_connection, id).await
    }

    async fn healthcheck(&self) -> Result<(), DbErr> {
        self.database_connection
            .execute_unprepared("SELECT 1;")
            .await
            .map(|_| ())
    }
}

impl Repository {
    #[must_use]
    pub fn new(connection: DatabaseConnection) -> Self {
        Repository {
            database_connection: connection,
        }
    }
}
