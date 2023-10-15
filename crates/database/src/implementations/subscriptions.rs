use async_trait::async_trait;
use entities::subscriptions::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    interfaces::subscriptions::{Error, Id, Payload, RepositoryTrait, Response},
    Repository,
};

impl From<Payload> for Model {
    fn from(value: Payload) -> Self {
        Model {
            id: value.id,
            user_id: value.user_id,
            subscriber_id: value.subscriber_id,
            created_at: value.created_at,
        }
    }
}

impl From<Model> for Response {
    fn from(value: Model) -> Self {
        Response {
            id: value.id,
            user_id: value.user_id,
            subscriber_id: value.subscriber_id,
            created_at: value.created_at,
        }
    }
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_subscription(&self, payload: Payload) -> Result<Response, Error> {
        let model: Model = payload.into();
        let active_model: ActiveModel = model.into();
        active_model
            .insert(&self.database_connection)
            .await
            .map(Into::into)
            .or(Err(Error::Unknown))
    }

    async fn delete_subscription(&self, id: Id) -> Result<(), Error> {
        Entity::delete_by_id(id)
            .exec(&self.database_connection)
            .await
            .map(|_| ())
            .or(Err(Error::Unknown))
    }
}
