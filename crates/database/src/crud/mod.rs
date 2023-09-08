use async_trait::async_trait;
#[allow(clippy::module_name_repetitions)]
pub use items::{
    CreatePayload as ItemsCreatePayload,
    Crud as ItemsCrud,
    Response as ItemsResponse,
    UpdatePayload as ItemsUpdatePayload,
};
use sea_orm::{
    ActiveModelBehavior,
    ActiveModelTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    IntoActiveModel,
    PrimaryKeyTrait,
};
#[allow(clippy::module_name_repetitions)]
pub use users::{
    CreatePayload as UsersCreatePayload,
    Crud as UsersCrud,
    Response as UsersResponse,
    UpdatePayload as UsersUpdatePayload,
};
#[allow(clippy::module_name_repetitions)]
pub use wishlists::{
    CreatePayload as WishlistsCreatePayload,
    Crud as WishlistsCrud,
    Response as WishlistsResponse,
    UpdatePayload as WishlistsUpdatePayload,
};

mod items;
mod users;
mod wishlists;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub(super) trait CrudTrait<T, Y>
where
    T: EntityTrait,
    Y: ActiveModelBehavior + Send + From<T::Model>,
    <Y::Entity as EntityTrait>::Model: IntoActiveModel<Y>,
{
    type Id: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static;
    type CreatePayload: Into<T::Model> + Sync + Send + 'static;
    type UpdatePayload;
    type Response: From<T::Model> + From<<<Y as ActiveModelTrait>::Entity as EntityTrait>::Model>;

    async fn create(
        database_connection: &DatabaseConnection,
        payload: Self::CreatePayload,
    ) -> Result<Self::Response, DbErr> {
        let model: T::Model = payload.into();
        let active_model: Y = model.into();
        active_model
            .insert(database_connection)
            .await
            .map(Into::into)
    }

    async fn get(
        database_connection: &DatabaseConnection,
        id: Self::Id,
    ) -> Result<Option<Self::Response>, DbErr> {
        T::find_by_id(id)
            .one(database_connection)
            .await
            .map(|x| x.map(Into::into))
    }

    async fn list(database_connection: &DatabaseConnection) -> Result<Vec<Self::Response>, DbErr> {
        T::find()
            .all(database_connection)
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn update(
        database_connection: &DatabaseConnection,
        id: Self::Id,
        payload: Self::UpdatePayload,
    ) -> Result<Self::Response, DbErr>;

    async fn delete(database_connection: &DatabaseConnection, id: Self::Id) -> Result<(), DbErr> {
        T::delete_by_id(id)
            .exec(database_connection)
            .await
            .map(|_| ())
    }
}
