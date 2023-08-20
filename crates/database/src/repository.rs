use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    ConnectionTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    Set,
};
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
        payload: items::create::DatabasePayload,
    ) -> Result<items::create::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();

        entities::items::ActiveModel {
            id: ActiveValue::Set(payload.id),
            wishlist_id: ActiveValue::Set(payload.wishlist_id),
            selected_by_id: ActiveValue::Set(None),
            name: ActiveValue::Set(payload.name),
            description: ActiveValue::Set(payload.description),
            price: ActiveValue::Set(payload.price),
            is_hidden: ActiveValue::Set(payload.is_hidden),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        }
        .insert(&self.database_connection)
        .await
        .map(std::convert::Into::into)
        .or(Err(DataError::Unknown))
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DataError> {
        entities::items::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(DataError::Unknown))
    }

    async fn get_item(&self, id: Uuid) -> Result<items::get::DatabaseResponse, DataError> {
        match entities::items::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response.into()),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_items(&self) -> Result<Vec<items::list::DatabaseResponse>, DataError> {
        entities::items::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
            .map(|x| x.into_iter().map(std::convert::Into::into).collect())
    }

    async fn update_item(
        &self,
        id: Uuid,
        payload: items::update::DatabasePayload,
    ) -> Result<items::update::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();

        let active_model = entities::items::ActiveModel {
            name: Set(payload.name),
            description: Set(payload.description),
            price: Set(payload.price),
            is_hidden: Set(payload.is_hidden),
            updated_at: Set(now),
            ..Default::default()
        };

        entities::items::Entity::update(active_model)
            .filter(entities::items::Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(std::convert::Into::into)
            .or(Err(DataError::Unknown))
    }

    async fn create_user(
        &self,
        payload: users::create::DatabasePayload,
    ) -> Result<users::create::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();

        entities::users::ActiveModel {
            id: ActiveValue::Set(payload.id),
            first_name: ActiveValue::Set(payload.first_name),
            second_name: ActiveValue::Set(payload.second_name),
            nick_name: ActiveValue::Set(payload.nick_name),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        }
        .insert(&self.database_connection)
        .await
        .map(std::convert::Into::into)
        .or(Err(DataError::Unknown))
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DataError> {
        entities::users::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(DataError::Unknown))
    }

    async fn get_user(&self, id: Uuid) -> Result<users::get::DatabaseResponse, DataError> {
        match entities::users::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response.into()),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_users(&self) -> Result<Vec<users::list::DatabaseResponse>, DataError> {
        entities::users::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
            .map(|x| x.into_iter().map(std::convert::Into::into).collect())
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: users::update::DatabasePayload,
    ) -> Result<users::update::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();

        let active_model = entities::users::ActiveModel {
            first_name: Set(payload.first_name),
            second_name: Set(payload.second_name),
            nick_name: Set(payload.nick_name),
            updated_at: Set(now),
            ..Default::default()
        };

        entities::users::Entity::update(active_model)
            .filter(entities::users::Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(std::convert::Into::into)
            .or(Err(DataError::Unknown))
    }

    async fn create_wishlist(
        &self,
        payload: wishlists::create::DatabasePayload,
    ) -> Result<wishlists::create::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();

        entities::wishlists::ActiveModel {
            id: ActiveValue::Set(payload.id),
            user_id: ActiveValue::Set(payload.user_id),
            name: ActiveValue::Set(payload.name),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        }
        .insert(&self.database_connection)
        .await
        .map(std::convert::Into::into)
        .or(Err(DataError::Unknown))
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DataError> {
        entities::wishlists::Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(DataError::Unknown))
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<wishlists::get::DatabaseResponse, DataError> {
        match entities::wishlists::Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
        {
            Ok(database_response) => match database_response {
                Some(response) => Ok(response.into()),
                None => Err(DataError::Unknown),
            },
            Err(_) => Err(DataError::Unknown),
        }
    }

    async fn list_wishlists(&self) -> Result<Vec<wishlists::list::DatabaseResponse>, DataError> {
        entities::wishlists::Entity::find()
            .all(&self.database_connection)
            .await
            .or(Err(DataError::Unknown))
            .map(|x| x.into_iter().map(std::convert::Into::into).collect())
    }

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: wishlists::update::DatabasePayload,
    ) -> Result<wishlists::update::DatabaseResponse, DataError> {
        let now = Utc::now().naive_utc();
        let active_model = entities::wishlists::ActiveModel {
            name: Set(payload.name),
            updated_at: Set(now),
            ..Default::default()
        };

        entities::wishlists::Entity::update(active_model)
            .filter(entities::wishlists::Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(std::convert::Into::into)
            .or(Err(DataError::Unknown))
    }

    async fn healthcheck(&self) -> Result<(), DataError> {
        self.database_connection
            .execute_unprepared("SELECT 1;")
            .await
            .map(|_| ())
            .or(Err(DataError::Unknown))
    }
}
