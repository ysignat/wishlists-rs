use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use entities::items::{Entity, Model};
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
    pub wishlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

#[derive(Deserialize)]
pub struct DatabaseUpdatePayload {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

pub struct DatabaseResponse {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub selected_by_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for DatabaseResponse {
    fn from(value: Model) -> Self {
        DatabaseResponse {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
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
            .exec(self.database_connection)
            .await
            .map(std::convert::Into::into)
    }
}
