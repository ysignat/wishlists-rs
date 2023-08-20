#![warn(clippy::pedantic)]
mod config;
mod utils;

use axum::Server;
use clap::Parser;
use config::Config;
use database::{connection::Connection, migrate};
use router::Router;
use utils::{get_bind_address, get_root_path, get_state};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let db_connection = Connection::new(
        config.postgres_url.clone(),
        config.postgres_pool_acquire_timeout,
        config.postgres_pool_size,
    )
    .connect()
    .await
    .expect("Cannot create connection pool");

    if config.migrate {
        migrate(&db_connection)
            .await
            .expect("Migration not successful");
    } else {
        let root_path = get_root_path(&config.app_root_path);
        let state = get_state(&config).await.unwrap();
        let bind_address = get_bind_address(&config.app_bind_address);
        let main_router = Router::new(root_path, state).build();

        Server::bind(&bind_address)
            .serve(main_router.into_make_service())
            .await
            .expect("Cannot start server");
    }
}
