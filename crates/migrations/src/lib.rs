pub use sea_orm_migration::prelude::*;

mod m20230726_210816_base;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230726_210816_base::Migration)]
    }
}
