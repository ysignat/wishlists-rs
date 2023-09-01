use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use entities::users::{Entity, Model};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    QueryFilter,
    Set,
};
use serde::Deserialize;
use uuid::Uuid;

use super::EntityCrudTrait;

#[derive(Deserialize)]
pub struct DatabaseCreatePayload {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}

#[derive(Deserialize)]
pub struct DatabaseUpdatePayload {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
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
impl EntityCrudTrait<Entity, Uuid, DatabaseCreatePayload, DatabaseUpdatePayload, DatabaseResponse>
    for EntityCrud<'_>
{
    fn get_database_connection(&self) -> &DatabaseConnection {
        self.database_connection
    }

    async fn create(&self, payload: DatabaseCreatePayload) -> Result<DatabaseResponse, DbErr> {
        let now = Utc::now().naive_utc();

        entities::users::ActiveModel {
            id: ActiveValue::Set(payload.id),
            first_name: ActiveValue::Set(payload.first_name),
            second_name: ActiveValue::Set(payload.second_name),
            nick_name: ActiveValue::Set(payload.nick_name),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        }
        .insert(self.database_connection)
        .await
        .map(std::convert::Into::into)
    }

    async fn update(
        &self,
        id: Uuid,
        payload: DatabaseUpdatePayload,
    ) -> Result<DatabaseResponse, DbErr> {
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
            .exec(self.database_connection)
            .await
            .map(std::convert::Into::into)
    }
}
