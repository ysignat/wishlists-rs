use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{
    errors::DataError,
    structs::{items, users, wishlists},
};

#[async_trait]
pub trait RepositoryTrait {
    async fn create_item(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: items::create::DatabasePayload,
    ) -> Result<entities::items::Model, DataError>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_item(&self, id: Uuid) -> Result<entities::items::Model, DataError>;

    async fn list_items(&self) -> Result<Vec<entities::items::Model>, DataError>;

    async fn update_item(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: items::update::DatabasePayload,
    ) -> Result<entities::items::Model, DataError>;

    async fn create_user(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: users::create::DatabasePayload,
    ) -> Result<entities::users::Model, DataError>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_user(&self, id: Uuid) -> Result<entities::users::Model, DataError>;

    async fn list_users(&self) -> Result<Vec<entities::users::Model>, DataError>;

    async fn update_user(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: users::update::DatabasePayload,
    ) -> Result<entities::users::Model, DataError>;

    async fn create_wishlist(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: wishlists::create::DatabasePayload,
    ) -> Result<entities::wishlists::Model, DataError>;

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DataError>;

    async fn get_wishlist(&self, id: Uuid) -> Result<entities::wishlists::Model, DataError>;

    async fn list_wishlists(&self) -> Result<Vec<entities::wishlists::Model>, DataError>;

    async fn update_wishlist(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: wishlists::update::DatabasePayload,
    ) -> Result<entities::wishlists::Model, DataError>;
}
