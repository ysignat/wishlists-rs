use async_trait::async_trait;
use chrono::Utc;
use entities::items::{Entity, Model};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    PrimaryKeyTrait,
    QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::structs::items;

#[async_trait]
trait Crud<T, U, V, W>
where
    T: EntityTrait,
    U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
{
    fn get_database_connection(&self) -> &DatabaseConnection;

    async fn create_item(&self, payload: V) -> T::Model;

    async fn get(&self, id: U) -> T::Model {
        let database_connection = self.get_database_connection();
        T::find_by_id(id)
            .one(database_connection)
            .await
            .unwrap()
            .unwrap()
    }

    async fn delete(&self, id: U) {
        let database_connection = self.get_database_connection();
        T::delete_by_id(id).exec(database_connection).await.unwrap();
    }

    async fn list(&self) -> Vec<T::Model> {
        T::find().all(self.get_database_connection()).await.unwrap()
    }

    async fn update_item(&self, id: U, payload: W) -> T::Model;
}

struct ItemsCrud {
    database_connection: DatabaseConnection,
}

#[async_trait]
impl Crud<Entity, Uuid, items::create::DatabasePayload, items::update::DatabasePayload>
    for ItemsCrud
{
    fn get_database_connection(&self) -> &DatabaseConnection {
        &self.database_connection
    }

    async fn create_item(&self, payload: items::create::DatabasePayload) -> Model {
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
        .unwrap()
    }

    async fn update_item(&self, id: Uuid, payload: items::update::DatabasePayload) -> Model {
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
            .exec(&self.database_connection)
            .await
            .unwrap()
    }
}
