#![warn(clippy::pedantic)]
mod config;
mod routers;
mod utils;

use axum::Server;
use clap::Parser;
use config::Config;
use database::connection::get_db_connection;
use migrations::{Migrator, MigratorTrait};
use routers::Router;
use utils::{get_bind_address, get_root_path, get_state};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let db_connection = get_db_connection(
        &config.postgres_url,
        config.postgres_pool_acquire_timeout,
        config.postgres_pool_size,
    )
    .await;

    if config.migrate {
        Migrator::up(&db_connection, None)
            .await
            .expect("Migration not successful");
    } else {
        let root_path = get_root_path(&config.app_root_path);
        let state = get_state(&config).await;
        let bind_address = get_bind_address(&config.app_bind_address);
        let main_router = Router::new(root_path, state).build();

        Server::bind(&bind_address)
            .serve(main_router.into_make_service())
            .await
            .expect("Cannot start server");
    }
}
