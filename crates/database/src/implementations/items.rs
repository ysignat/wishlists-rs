use async_trait::async_trait;
use entities::items::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    interfaces::items::{Error, Id, Payload, Predicate, RepositoryTrait, Response},
    Repository,
};

impl From<Payload> for Model {
    fn from(value: Payload) -> Self {
        Model {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            picture_id: value.picture_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Model> for Response {
    fn from(value: Model) -> Self {
        Response {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            picture_id: value.picture_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_item(&self, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();
        active_model
            .insert(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn get_item(&self, id: Id) -> Result<Option<Response>, Error> {
        Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
            .map(|x| x.map(Into::into))
            .or(Err(Error::Unknown))
    }

    async fn list_items(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error> {
        match predicate {
            Some(value) => Entity::find().filter(Column::Name.contains(value)),
            None => Entity::find(),
        }
        .order_by_desc(Column::CreatedAt)
        .order_by_desc(Column::Id)
        .all(&self.database_connection)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())
        .or(Err(Error::Unknown))
    }

    async fn update_item(&self, id: Id, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();

        Entity::update(active_model)
            .filter(Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn delete_item(&self, id: Id) -> Result<(), Error> {
        Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }
}
