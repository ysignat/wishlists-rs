use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use super::{
    errors::DataError,
    repository_trait::RepositoryTrait,
    structs::{items, users, wishlists},
};

pub struct Repository {
    pub database_connection: DatabaseConnection,
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_item(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: items::create::DatabasePayload,
    ) -> Result<entities::items::Model, DataError> {
        entities::items::ActiveModel {
            id: ActiveValue::Set(uuid),
            wishlist_id: ActiveValue::Set(payload.wishlist_id),
            selected_by_id: ActiveValue::Set(None),
            name: ActiveValue::Set(payload.name),
            description: ActiveValue::Set(payload.description),
            price: ActiveValue::Set(payload.price),
            is_hidden: ActiveValue::Set(payload.is_hidden),
            created_at: ActiveValue::Set(timestamp),
            updated_at: ActiveValue::Set(timestamp),
        }
        .insert(&self.database_connection)
        .await
        .or(Err(DataError::Unknown))
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DataError> {
        match entities::items::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn get_item(&self, id: Uuid) -> Result<entities::items::Model, DataError> {
        match entities::items::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_items(&self) -> Result<Vec<entities::items::Model>, DataError> {
        entities::items::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
    }

    async fn update_item(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: items::update::DatabasePayload,
    ) -> Result<entities::items::Model, DataError> {
        match entities::items::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => {
                    let mut active_model: entities::items::ActiveModel = response.into();

                    active_model.name = Set(payload.name);
                    active_model.description = Set(payload.description);
                    active_model.price = Set(payload.price);
                    active_model.is_hidden = Set(payload.is_hidden);
                    active_model.updated_at = Set(timestamp);

                    active_model
                        .update(&self.database_connection)
                        .await
                        .or(Err(DataError::Unknown))
                }
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn create_user(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: users::create::DatabasePayload,
    ) -> Result<entities::users::Model, DataError> {
        entities::users::ActiveModel {
            id: ActiveValue::Set(uuid),
            first_name: ActiveValue::Set(payload.first_name),
            second_name: ActiveValue::Set(payload.second_name),
            nick_name: ActiveValue::Set(payload.nick_name),
            created_at: ActiveValue::Set(timestamp),
            updated_at: ActiveValue::Set(timestamp),
        }
        .insert(&self.database_connection)
        .await
        .or(Err(DataError::Unknown))
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DataError> {
        match entities::users::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn get_user(&self, id: Uuid) -> Result<entities::users::Model, DataError> {
        match entities::users::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_users(&self) -> Result<Vec<entities::users::Model>, DataError> {
        entities::users::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
    }

    async fn update_user(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: users::update::DatabasePayload,
    ) -> Result<entities::users::Model, DataError> {
        match entities::users::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => {
                    let mut active_model: entities::users::ActiveModel = response.into();

                    active_model.first_name = Set(payload.first_name);
                    active_model.second_name = Set(payload.second_name);
                    active_model.updated_at = Set(timestamp);

                    active_model
                        .update(&self.database_connection)
                        .await
                        .or(Err(DataError::Unknown))
                }
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn create_wishlist(
        &self,
        uuid: Uuid,
        timestamp: NaiveDateTime,
        payload: wishlists::create::DatabasePayload,
    ) -> Result<entities::wishlists::Model, DataError> {
        entities::wishlists::ActiveModel {
            id: ActiveValue::Set(uuid),
            user_id: ActiveValue::Set(payload.user_id),
            name: ActiveValue::Set(payload.name),
            created_at: ActiveValue::Set(timestamp),
            updated_at: ActiveValue::Set(timestamp),
        }
        .insert(&self.database_connection)
        .await
        .or(Err(DataError::Unknown))
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DataError> {
        match entities::wishlists::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<entities::wishlists::Model, DataError> {
        match entities::wishlists::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_wishlists(&self) -> Result<Vec<entities::wishlists::Model>, DataError> {
        entities::wishlists::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
    }

    async fn update_wishlist(
        &self,
        timestamp: NaiveDateTime,
        id: Uuid,
        payload: wishlists::update::DatabasePayload,
    ) -> Result<entities::wishlists::Model, DataError> {
        match entities::wishlists::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => {
                    let mut active_model: entities::wishlists::ActiveModel = response.into();

                    active_model.name = Set(payload.name);
                    active_model.updated_at = Set(timestamp);

                    active_model
                        .update(&self.database_connection)
                        .await
                        .or(Err(DataError::Unknown))
                }
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }
}
