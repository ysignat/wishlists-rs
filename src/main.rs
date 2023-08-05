#![warn(clippy::pedantic)]
mod config;
mod handlers;
mod utils;

use axum::Router;
use clap::Parser;
use config::Config;
use handlers::{health, users, wishlists};
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{net::SocketAddr, time::Duration};
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

        let app = Router::new()
            .merge(users::get_router(&root_path, app_state.clone()))
            .merge(wishlists::get_router(&root_path, app_state))
            .merge(health::get_router(&root_path));

        let addr: SocketAddr = config
            .app_bind_address
            .parse()
            .unwrap_or_else(|_| panic!("Invalid bind address: {}", config.app_bind_address));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Cannot start server");
    }
}
