use async_trait::async_trait;
use chrono::NaiveDateTime;
pub use entities::wishlists::Model as Response;
use entities::wishlists::{ActiveModel, Entity, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::CrudTrait;

#[derive(Deserialize)]
pub struct CreatePayload {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl From<CreatePayload> for Model {
    fn from(value: CreatePayload) -> Self {
        Model {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

pub struct Crud;

#[async_trait]
impl CrudTrait<Entity, ActiveModel> for Crud {
    type Id = Uuid;
    type CreatePayload = CreatePayload;
    type UpdatePayload = UpdatePayload;
    type Response = Response;

    async fn update(
        database_connection: &DatabaseConnection,
        id: Self::Id,
        payload: Self::UpdatePayload,
    ) -> Result<Self::Response, DbErr> {
        let active_model = entities::wishlists::ActiveModel {
            name: Set(payload.name),
            updated_at: Set(payload.updated_at),
            ..Default::default()
        };

        entities::wishlists::Entity::update(active_model)
            .filter(entities::wishlists::Column::Id.eq(id))
            .exec(database_connection)
            .await
            .map(Into::into)
    }
}
