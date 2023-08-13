#![warn(clippy::pedantic)]
mod config;
mod handlers;
mod structs;
mod utils;

use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use handlers::{users, wishlists};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(config.postgres_pool_size)
        .acquire_timeout(Duration::from_secs(config.postgres_pool_acquire_timeout))
        .connect(&config.postgres_url)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Cannot create connection pool for URL: {}",
                config.postgres_url
            )
        });

    if config.migrate {
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Migration not successful");
    } else {
        let root_path = if config.app_root_path == "/" {
            String::new()
        } else {
            config.app_root_path
        };

        let app = Router::new()
            .route(
                &format!("{root_path}/users"),
                get(users::list).post(users::create),
            )
            .route(
                &format!("{root_path}/users/:id"),
                get(users::get).put(users::update).delete(users::delete),
            )
            .route(
                &format!("{root_path}/wishlists"),
                get(wishlists::list).post(wishlists::create),
            )
            .route(
                &format!("{root_path}/wishlists/:id"),
                get(wishlists::get)
                    .put(wishlists::update)
                    .delete(wishlists::delete),
            )
            .with_state(pool);

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
