#![warn(clippy::pedantic)]
use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};
use uuid::Uuid;

use crate::crud::{items, users, wishlists, CrudTrait};

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait RepositoryTrait {
    async fn create_item(
        &self,
        payload: items::DatabaseCreatePayload,
    ) -> Result<items::DatabaseResponse, DbErr>;

    async fn get_item(&self, id: Uuid) -> Result<Option<items::DatabaseResponse>, DbErr>;

    async fn list_items(&self) -> Result<Vec<items::DatabaseResponse>, DbErr>;

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::DatabaseUpdatePayload,
    ) -> Result<items::DatabaseResponse, DbErr>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_user(
        &self,
        payload: users::DatabaseCreatePayload,
    ) -> Result<users::DatabaseResponse, DbErr>;

    async fn get_user(&self, id: Uuid) -> Result<Option<users::DatabaseResponse>, DbErr>;

    async fn list_users(&self) -> Result<Vec<users::DatabaseResponse>, DbErr>;

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::DatabaseUpdatePayload,
    ) -> Result<users::DatabaseResponse, DbErr>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_wishlist(
        &self,
        payload: wishlists::DatabaseCreatePayload,
    ) -> Result<wishlists::DatabaseResponse, DbErr>;

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<wishlists::DatabaseResponse>, DbErr>;

    async fn list_wishlists(&self) -> Result<Vec<wishlists::DatabaseResponse>, DbErr>;

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::DatabaseUpdatePayload,
    ) -> Result<wishlists::DatabaseResponse, DbErr>;

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
        payload: items::DatabaseCreatePayload,
    ) -> Result<items::DatabaseResponse, DbErr> {
        items::Crud::create(&self.database_connection, payload).await
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<items::DatabaseResponse>, DbErr> {
        items::Crud::get(&self.database_connection, id).await
    }

    async fn list_items(&self) -> Result<Vec<items::DatabaseResponse>, DbErr> {
        items::Crud::list(&self.database_connection).await
    }

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::DatabaseUpdatePayload,
    ) -> Result<items::DatabaseResponse, DbErr> {
        items::Crud::update(&self.database_connection, id, payload).await
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr> {
        items::Crud::delete(&self.database_connection, id).await
    }

    async fn create_user(
        &self,
        payload: users::DatabaseCreatePayload,
    ) -> Result<users::DatabaseResponse, DbErr> {
        users::Crud::create(&self.database_connection, payload).await
    }

    async fn get_user(&self, id: Uuid) -> Result<Option<users::DatabaseResponse>, DbErr> {
        users::Crud::get(&self.database_connection, id).await
    }

    async fn list_users(&self) -> Result<Vec<users::DatabaseResponse>, DbErr> {
        users::Crud::list(&self.database_connection).await
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::DatabaseUpdatePayload,
    ) -> Result<users::DatabaseResponse, DbErr> {
        users::Crud::update(&self.database_connection, id, payload).await
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr> {
        users::Crud::delete(&self.database_connection, id).await
    }

    async fn create_wishlist(
        &self,
        payload: wishlists::DatabaseCreatePayload,
    ) -> Result<wishlists::DatabaseResponse, DbErr> {
        wishlists::Crud::create(&self.database_connection, payload).await
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<wishlists::DatabaseResponse>, DbErr> {
        wishlists::Crud::get(&self.database_connection, id).await
    }

    async fn list_wishlists(&self) -> Result<Vec<wishlists::DatabaseResponse>, DbErr> {
        wishlists::Crud::list(&self.database_connection).await
    }

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::DatabaseUpdatePayload,
    ) -> Result<wishlists::DatabaseResponse, DbErr> {
        wishlists::Crud::update(&self.database_connection, id, payload).await
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr> {
        wishlists::Crud::delete(&self.database_connection, id).await
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
