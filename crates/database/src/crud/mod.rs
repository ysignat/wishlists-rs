use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior,
    ActiveModelTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    IntoActiveModel,
    PrimaryKeyTrait,
};

pub mod items;
pub mod users;
pub mod wishlists;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub(crate) trait CrudTrait<T, Y>
where
    T: EntityTrait,
    Y: ActiveModelBehavior + Send + From<T::Model>,
    <Y::Entity as EntityTrait>::Model: IntoActiveModel<Y>,
{
    type Id: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static;
    type CreatePayload: Into<T::Model> + Sync + Send + 'static;
    type UpdatePayload;
    type Response: From<T::Model> + From<<<Y as ActiveModelTrait>::Entity as EntityTrait>::Model>;

    fn get_database_connection(&self) -> &DatabaseConnection;

    async fn create(&self, payload: Self::CreatePayload) -> Result<Self::Response, DbErr> {
        let database_connection = self.get_database_connection();
        let model: T::Model = payload.into();
        let active_model: Y = model.into();
        active_model
            .insert(database_connection)
            .await
            .map(Into::into)
    }

    async fn get(&self, id: Self::Id) -> Result<Option<Self::Response>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find_by_id(id)
            .one(database_connection)
            .await
            .map(|x| x.map(Into::into))
    }

    async fn list(&self) -> Result<Vec<Self::Response>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find()
            .all(database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn update(
        &self,
        id: Self::Id,
        payload: Self::UpdatePayload,
    ) -> Result<Self::Response, DbErr>;

    async fn delete(&self, id: Self::Id) -> Result<(), DbErr> {
        let database_connection = self.get_database_connection();
        T::delete_by_id(id)
            .exec(database_connection)
            .await
            .map(|_| ())
    }
}
