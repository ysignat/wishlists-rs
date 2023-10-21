use async_trait::async_trait;
use entities::users::{ActiveModel, Column, Entity, Model};
use migrations::{Expr, Query};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    interfaces::{
        users::{Error, Id, Payload, Predicate, RepositoryTrait, Response},
        wishlists,
    },
    Repository,
};

impl From<Payload> for Model {
    fn from(value: Payload) -> Self {
        Model {
            id: value.id,
            name: value.name,
            avatar_id: value.avatar_id,
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
            avatar_id: value.avatar_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_user(&self, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();
        active_model
            .insert(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn get_user(&self, id: Id) -> Result<Option<Response>, Error> {
        Entity::find_by_id(id)
            .one(&self.database_connection)
            .await
            .map(|x| x.map(Into::into))
            .or(Err(Error::Unknown))
    }

    async fn list_users(&self, predicate: Option<Predicate>) -> Result<Vec<Response>, Error> {
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

    async fn update_user(&self, id: Id, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();

        Entity::update(active_model)
            .filter(Column::Id.eq(id))
            .exec(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn delete_user(&self, id: Id) -> Result<(), Error> {
        Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }

    async fn list_user_wishlists(
        &self,
        id: Id,
        predicate: Option<wishlists::Predicate>,
    ) -> Result<Vec<wishlists::Response>, Error> {
        let condition = Condition::all()
            .add(entities::wishlists::Column::UserId.eq(id))
            .add(entities::wishlists::Column::Name.like(predicate.unwrap_or_default()));

        entities::wishlists::Entity::find()
            .filter(condition)
            .order_by_desc(entities::wishlists::Column::UpdatedAt)
            .all(&self.database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .or(Err(Error::Unknown))
    }

    async fn list_user_subscribers(
        &self,
        id: Id,
        predicate: Option<Predicate>,
    ) -> Result<Vec<Response>, Error> {
        let condition = Condition::all()
            .add(
                Column::Id.in_subquery(
                    Query::select()
                        .expr(Expr::col(entities::subscriptions::Column::SubscriberId))
                        .and_where(entities::subscriptions::Column::UserId.eq(id))
                        .clone(),
                ),
            )
            .add(Column::Name.like(predicate.unwrap_or_default()));

        Entity::find()
            .filter(Condition::any().add(condition))
            .order_by_desc(Column::Name)
            .all(&self.database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .or(Err(Error::Unknown))
    }

    async fn list_user_subscriptions(
        &self,
        id: Id,
        predicate: Option<Predicate>,
    ) -> Result<Vec<Response>, Error> {
        let condition = Condition::all()
            .add(
                Column::Id.in_subquery(
                    Query::select()
                        .expr(Expr::col(entities::subscriptions::Column::UserId))
                        .and_where(entities::subscriptions::Column::SubscriberId.eq(id))
                        .from(entities::subscriptions::Entity)
                        .clone(),
                ),
            )
            .add(Column::Name.like(predicate.unwrap_or_default()));

        Entity::find()
            .filter(Condition::any().add(condition))
            .order_by_desc(Column::Name)
            .all(&self.database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .or(Err(Error::Unknown))
    }
}
