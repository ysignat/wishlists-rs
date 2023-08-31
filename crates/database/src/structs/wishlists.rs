use async_trait::async_trait;
use chrono::Utc;
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

use crate::EntityCrud;

#[derive(Deserialize)]
pub struct CreatePayload {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    pub name: String,
}

struct WishlistsCrud<'a> {
    database_connection: &'a DatabaseConnection,
}

#[async_trait]
impl EntityCrud<Entity, Uuid, CreatePayload, UpdatePayload> for WishlistsCrud<'_> {
    fn get_database_connection(&self) -> &DatabaseConnection {
        self.database_connection
    }

    async fn create(&self, payload: CreatePayload) -> Result<Model, DbErr> {
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
    }

    async fn update(&self, id: Uuid, payload: UpdatePayload) -> Result<Model, DbErr> {
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
    }
}
