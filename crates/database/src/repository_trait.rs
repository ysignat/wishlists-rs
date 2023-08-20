use async_trait::async_trait;
use uuid::Uuid;

use super::{
    errors::DataError,
    structs::{items, users, wishlists},
};

#[async_trait]
pub trait RepositoryTrait {
    async fn create_item(
        &self,
        payload: items::create::DatabasePayload,
    ) -> Result<items::create::DatabaseResponse, DataError>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_item(&self, id: Uuid) -> Result<items::get::DatabaseResponse, DataError>;

    async fn list_items(&self) -> Result<Vec<items::list::DatabaseResponse>, DataError>;

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::update::DatabasePayload,
    ) -> Result<items::update::DatabaseResponse, DataError>;

    async fn create_user(
        &self,
        payload: users::create::DatabasePayload,
    ) -> Result<users::create::DatabaseResponse, DataError>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_user(&self, id: Uuid) -> Result<users::get::DatabaseResponse, DataError>;

    async fn list_users(&self) -> Result<Vec<users::list::DatabaseResponse>, DataError>;

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::update::DatabasePayload,
    ) -> Result<users::update::DatabaseResponse, DataError>;

    async fn create_wishlist(
        &self,
        payload: wishlists::create::DatabasePayload,
    ) -> Result<wishlists::create::DatabaseResponse, DataError>;

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_wishlist(&self, id: Uuid) -> Result<wishlists::get::DatabaseResponse, DataError>;

    async fn list_wishlists(&self) -> Result<Vec<wishlists::list::DatabaseResponse>, DataError>;

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::update::DatabasePayload,
    ) -> Result<wishlists::update::DatabaseResponse, DataError>;

    async fn healthcheck(&self) -> Result<(), DataError>;
}
