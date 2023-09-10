#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use async_trait::async_trait;
use crud::{CrudTrait, ItemsCrud, UsersCrud, WishlistsCrud};
pub use crud::{
    ItemsCreatePayload,
    ItemsResponse,
    ItemsUpdatePayload,
    UsersCreatePayload,
    UsersResponse,
    UsersUpdatePayload,
    WishlistsCreatePayload,
    WishlistsResponse,
    WishlistsUpdatePayload,
};
pub use migrations::{Migrator, MigratorTrait};
use sea_orm::ConnectionTrait;
pub use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use uuid::Uuid;

mod crud;

#[async_trait]
pub trait RepositoryTrait {
    async fn create_item(&self, payload: ItemsCreatePayload) -> Result<ItemsResponse, DbErr>;

    async fn get_item(&self, id: Uuid) -> Result<Option<ItemsResponse>, DbErr>;

    async fn list_items(&self) -> Result<Vec<ItemsResponse>, DbErr>;

    async fn update_item(
        &self,
        id: Uuid,
        payload: ItemsUpdatePayload,
    ) -> Result<ItemsResponse, DbErr>;

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_user(&self, payload: UsersCreatePayload) -> Result<UsersResponse, DbErr>;

    async fn get_user(&self, id: Uuid) -> Result<Option<UsersResponse>, DbErr>;

    async fn list_users(&self) -> Result<Vec<UsersResponse>, DbErr>;

    async fn update_user(
        &self,
        id: Uuid,
        payload: UsersUpdatePayload,
    ) -> Result<UsersResponse, DbErr>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr>;

    async fn create_wishlist(
        &self,
        payload: WishlistsCreatePayload,
    ) -> Result<WishlistsResponse, DbErr>;

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<WishlistsResponse>, DbErr>;

    async fn list_wishlists(&self) -> Result<Vec<WishlistsResponse>, DbErr>;

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: WishlistsUpdatePayload,
    ) -> Result<WishlistsResponse, DbErr>;

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr>;

    async fn healthcheck(&self) -> Result<(), DbErr>;
}

pub struct Repository {
    database_connection: DatabaseConnection,
}

#[async_trait]
impl RepositoryTrait for Repository {
    async fn create_item(&self, payload: ItemsCreatePayload) -> Result<ItemsResponse, DbErr> {
        ItemsCrud::create(&self.database_connection, payload).await
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<ItemsResponse>, DbErr> {
        ItemsCrud::get(&self.database_connection, id).await
    }

    async fn list_items(&self) -> Result<Vec<ItemsResponse>, DbErr> {
        ItemsCrud::list(&self.database_connection).await
    }

    async fn update_item(
        &self,
        id: Uuid,
        payload: ItemsUpdatePayload,
    ) -> Result<ItemsResponse, DbErr> {
        ItemsCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), DbErr> {
        ItemsCrud::delete(&self.database_connection, id).await
    }

    async fn create_user(&self, payload: UsersCreatePayload) -> Result<UsersResponse, DbErr> {
        UsersCrud::create(&self.database_connection, payload).await
    }

    async fn get_user(&self, id: Uuid) -> Result<Option<UsersResponse>, DbErr> {
        UsersCrud::get(&self.database_connection, id).await
    }

    async fn list_users(&self) -> Result<Vec<UsersResponse>, DbErr> {
        UsersCrud::list(&self.database_connection).await
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: UsersUpdatePayload,
    ) -> Result<UsersResponse, DbErr> {
        UsersCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), DbErr> {
        UsersCrud::delete(&self.database_connection, id).await
    }

    async fn create_wishlist(
        &self,
        payload: WishlistsCreatePayload,
    ) -> Result<WishlistsResponse, DbErr> {
        WishlistsCrud::create(&self.database_connection, payload).await
    }

    async fn get_wishlist(&self, id: Uuid) -> Result<Option<WishlistsResponse>, DbErr> {
        WishlistsCrud::get(&self.database_connection, id).await
    }

    async fn list_wishlists(&self) -> Result<Vec<WishlistsResponse>, DbErr> {
        WishlistsCrud::list(&self.database_connection).await
    }

    async fn update_wishlist(
        &self,
        id: Uuid,
        payload: WishlistsUpdatePayload,
    ) -> Result<WishlistsResponse, DbErr> {
        WishlistsCrud::update(&self.database_connection, id, payload).await
    }

    async fn delete_wishlist(&self, id: Uuid) -> Result<(), DbErr> {
        WishlistsCrud::delete(&self.database_connection, id).await
    }

    async fn healthcheck(&self) -> Result<(), DbErr> {
        self.database_connection
            .execute_unprepared("SELECT 1;")
            .await
            .map(|_| ())
    }
}

impl Repository {
    #[must_use]
    pub fn new(connection: DatabaseConnection) -> Self {
        Repository {
            database_connection: connection,
        }
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{
        entity::{prelude::*, *},
        tests_cfg::*,
        DatabaseBackend,
        MockDatabase,
        Transaction,
    };

    #[async_std::test]
    async fn test_find_cake() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![cake::Model {
                    id: 1,
                    name: "New York Cheese".to_owned(),
                }],
                // Second query result
                vec![
                    cake::Model {
                        id: 1,
                        name: "New York Cheese".to_owned(),
                    },
                    cake::Model {
                        id: 2,
                        name: "Chocolate Forest".to_owned(),
                    },
                ],
            ])
            .append_query_results([
                // Third query result
                [(
                    cake::Model {
                        id: 1,
                        name: "Apple Cake".to_owned(),
                    },
                    fruit::Model {
                        id: 2,
                        name: "Apple".to_owned(),
                        cake_id: Some(1),
                    },
                )],
            ])
            .into_connection();

        // Find a cake from MockDatabase
        // Return the first query result
        assert_eq!(
            cake::Entity::find().one(&db).await?,
            Some(cake::Model {
                id: 1,
                name: "New York Cheese".to_owned(),
            })
        );

        // Find all cakes from MockDatabase
        // Return the second query result
        assert_eq!(
            cake::Entity::find().all(&db).await?,
            [
                cake::Model {
                    id: 1,
                    name: "New York Cheese".to_owned(),
                },
                cake::Model {
                    id: 2,
                    name: "Chocolate Forest".to_owned(),
                },
            ]
        );

        // Find all cakes with its related fruits
        assert_eq!(
            cake::Entity::find()
                .find_also_related(fruit::Entity)
                .all(&db)
                .await?,
            [(
                cake::Model {
                    id: 1,
                    name: "Apple Cake".to_owned(),
                },
                Some(fruit::Model {
                    id: 2,
                    name: "Apple".to_owned(),
                    cake_id: Some(1),
                })
            )]
        );

        // Checking transaction log
        assert_eq!(
            db.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
                    [1u64.into()]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
                    []
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name", "fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id" FROM "cake" LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
                    []
                ),
            ]
        );

        Ok(())
    }
}
