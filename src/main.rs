#![warn(clippy::pedantic)]
mod config;
mod routers;
mod utils;

use std::{net::SocketAddr, time::Duration};

use axum::Server;
use clap::Parser;
use config::Config;
use migrations::{Migrator, MigratorTrait};
use routers::Router;
use sea_orm::{ConnectOptions, Database};
use utils::AppState;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let mut connection_opts = ConnectOptions::new(config.postgres_url.clone());
    connection_opts
        .acquire_timeout(Duration::from_secs(config.postgres_pool_acquire_timeout))
        .max_connections(config.postgres_pool_size);

    let db = Database::connect(connection_opts)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Cannot create connection pool for URL: {}",
                &config.postgres_url
            )
        });

    if config.migrate {
        Migrator::up(&db, None)
            .await
            .expect("Migration not successful");
    } else {
        let root_path = if config.app_root_path == "/" {
            String::new()
        } else {
            config.app_root_path
        };

        let app_state = AppState {
            postgres_connection: db,
        };

        let addr: SocketAddr = config
            .app_bind_address
            .parse()
            .unwrap_or_else(|_| panic!("Invalid bind address: {}", config.app_bind_address));

        let main_router = Router::new(root_path, app_state).build();

        Server::bind(&addr)
            .serve(main_router.into_make_service())
            .await
            .expect("Cannot start server");
    }
}
