use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use entities::wishlists::{Entity, Model};
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
    pub name: String,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct DatabaseUpdatePayload {
    pub name: String,
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

        entities::wishlists::ActiveModel {
            id: ActiveValue::Set(payload.id),
            user_id: ActiveValue::Set(payload.user_id),
            name: ActiveValue::Set(payload.name),
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
        let active_model = entities::wishlists::ActiveModel {
            name: Set(payload.name),
            updated_at: Set(now),
            ..Default::default()
        };

        entities::wishlists::Entity::update(active_model)
            .filter(entities::wishlists::Column::Id.eq(id))
            .exec(self.database_connection)
            .await
            .map(std::convert::Into::into)
    }
}
