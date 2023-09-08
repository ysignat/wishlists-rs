use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::items::{ActiveModel, Entity, Model};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use super::CrudTrait;

#[derive(Deserialize)]
pub struct DatabaseCreatePayload {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub created_at: NaiveDateTime,
}

impl From<DatabaseCreatePayload> for Model {
    fn from(value: DatabaseCreatePayload) -> Self {
        Model {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: None,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            created_at: value.created_at,
            updated_at: value.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct DatabaseUpdatePayload {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
    pub updated_at: NaiveDateTime,
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

pub(crate) struct Crud;

#[async_trait]
impl CrudTrait<Entity, ActiveModel> for Crud {
    type Id = Uuid;
    type CreatePayload = DatabaseCreatePayload;
    type UpdatePayload = DatabaseUpdatePayload;
    type Response = DatabaseResponse;

    async fn update(
        database_connection: &DatabaseConnection,
        id: Self::Id,
        payload: Self::UpdatePayload,
    ) -> Result<Self::Response, DbErr> {
        let active_model = entities::items::ActiveModel {
            name: Set(payload.name),
            description: Set(payload.description),
            price: Set(payload.price),
            is_hidden: Set(payload.is_hidden),
            updated_at: Set(payload.updated_at),
            ..Default::default()
        };

        entities::items::Entity::update(active_model)
            .filter(entities::items::Column::Id.eq(id))
            .exec(database_connection)
            .await
            .map(Into::into)
    }
}
