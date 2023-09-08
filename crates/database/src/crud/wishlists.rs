use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::wishlists::{ActiveModel, Entity, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::EntityCrudTrait;

#[derive(Deserialize)]
pub struct DatabaseCreatePayload {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl From<DatabaseCreatePayload> for Model {
    fn from(value: DatabaseCreatePayload) -> Self {
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
pub struct DatabaseUpdatePayload {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

pub struct DatabaseResponse {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub(crate) struct EntityCrud;

#[async_trait]
impl EntityCrudTrait<Entity, ActiveModel> for EntityCrud {
    type Id = Uuid;
    type CreatePayload = DatabaseCreatePayload;
    type UpdatePayload = DatabaseUpdatePayload;
    type Response = DatabaseResponse;

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
