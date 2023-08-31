use async_trait::async_trait;
use chrono::Utc;
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

use crate::EntityCrud;

#[derive(Deserialize)]
pub struct CreatePayload {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}

struct UsersCrud<'a> {
    database_connection: &'a DatabaseConnection,
}

#[async_trait]
impl EntityCrud<Entity, Uuid, CreatePayload, UpdatePayload> for UsersCrud<'_> {
    fn get_database_connection(&self) -> &DatabaseConnection {
        self.database_connection
    }

    async fn create(&self, payload: CreatePayload) -> Result<Model, DbErr> {
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
    }

    async fn update(&self, id: Uuid, payload: UpdatePayload) -> Result<Model, DbErr> {
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
    }
}
