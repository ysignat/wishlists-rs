use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::users::{ActiveModel, Entity, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::EntityCrudTrait;

#[derive(Deserialize)]
pub struct DatabaseCreatePayload {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
    pub created_at: NaiveDateTime,
}

impl From<DatabaseCreatePayload> for Model {
    fn from(value: DatabaseCreatePayload) -> Self {
        Model {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct DatabaseUpdatePayload {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
    pub updated_at: NaiveDateTime,
}

pub struct DatabaseResponse {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct EntityCrud<'a> {
    pub database_connection: &'a DatabaseConnection,
}

#[async_trait]
impl EntityCrudTrait<Entity, ActiveModel> for EntityCrud<'_> {
    type Id = Uuid;
    type CreatePayload = DatabaseCreatePayload;
    type UpdatePayload = DatabaseUpdatePayload;
    type Response = DatabaseResponse;

    fn get_database_connection(&self) -> &DatabaseConnection {
        self.database_connection
    }

    async fn update(
        &self,
        id: Self::Id,
        payload: Self::UpdatePayload,
    ) -> Result<Self::Response, DbErr> {
        let active_model = entities::users::ActiveModel {
            first_name: Set(payload.first_name),
            second_name: Set(payload.second_name),
            nick_name: Set(payload.nick_name),
            updated_at: Set(payload.updated_at),
            ..Default::default()
        };

        entities::users::Entity::update(active_model)
            .filter(entities::users::Column::Id.eq(id))
            .exec(self.database_connection)
            .await
            .map(Into::into)
    }
}
