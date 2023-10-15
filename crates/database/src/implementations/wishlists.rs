use async_trait::async_trait;
use entities::wishlists::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    Condition,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};

use crate::interfaces::{
    items,
    wishlists::{Error, Id, Payload, Predicate, RepositoryTrait, Response},
};

impl From<Payload> for Model {
    fn from(value: Payload) -> Self {
        Model {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Model> for Response {
    fn from(value: Model) -> Self {
        Response {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

struct Repository {
    database_connection: DatabaseConnection,
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_wishlist(&self, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();
        active_model
            .insert(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn get_wishlist(&self, id: Id) -> Result<Option<Response>, Error> {
        Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
            .map(|x| x.map(Into::into))
            .or(Err(Error::Unknown))
    }

    async fn list_wishlists(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error> {
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

    async fn update_wishlist(&self, id: Id, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();

        Entity::update(active_model)
            .filter(Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn delete_wishlist(&self, id: Id) -> Result<(), Error> {
        Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }

    async fn list_wishlist_items(
        &self,
        id: Id,
        predicate: Option<items::Predicate>,
    ) -> Result<Vec<items::Response>, Error> {
        let condition = Condition::all()
            .add(entities::items::Column::WishlistId.eq(id))
            .add(entities::items::Column::Name.like(predicate.unwrap_or("")));

        entities::items::Entity::find()
            .filter(condition)
            .order_by_desc(entities::items::Column::UpdatedAt)
            .order_by_desc(entities::items::Column::Id)
            .all(&self.database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .or(Err(Error::Unknown))
    }
}
