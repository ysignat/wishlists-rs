use async_trait::async_trait;
use chrono::Utc;
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

use crate::EntityCrud;

#[derive(Deserialize)]
pub struct CreatePayload {
    pub id: Uuid,
    pub wishlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

struct ItemsCrud<'a> {
    database_connection: &'a DatabaseConnection,
}

#[async_trait]
impl EntityCrud<Entity, Uuid, CreatePayload, UpdatePayload> for ItemsCrud<'_> {
    fn get_database_connection(&self) -> &DatabaseConnection {
        self.database_connection
    }

    async fn create(&self, payload: CreatePayload) -> Result<Model, DbErr> {
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
        .insert(self.get_database_connection())
        .await
    }

    async fn update(&self, id: Uuid, payload: UpdatePayload) -> Result<Model, DbErr> {
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
    }
}
