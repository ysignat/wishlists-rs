use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PrimaryKeyTrait};

pub mod items;
pub mod users;
pub mod wishlists;

#[async_trait]
pub trait EntityCrudTrait<T, U, V, W, X>
where
    T: EntityTrait,
    U: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + 'static,
    X: From<T::Model>,
{
    fn get_database_connection(&self) -> &DatabaseConnection;

    async fn create(&self, payload: V) -> Result<X, DbErr>;

    async fn get(&self, id: U) -> Result<Option<X>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find_by_id(id)
            .one(database_connection)
            .await
            .map(|x| x.map(std::convert::Into::into))
    }

    async fn list(&self) -> Result<Vec<X>, DbErr> {
        let database_connection = self.get_database_connection();
        T::find()
            .all(database_connection)
            .await
            .map(|x| x.into_iter().map(std::convert::Into::into).collect())
    }

    async fn update(&self, id: U, payload: W) -> Result<X, DbErr>;

    async fn delete(&self, id: U) -> Result<(), DbErr> {
        let database_connection = self.get_database_connection();
        T::delete_by_id(id)
            .exec(database_connection)
            .await
            .map(|_| ())
    }
}
