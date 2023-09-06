#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
mod config;

use axum::{Router as AxumRouter, Server};
use clap::Parser;
use config::{Commands, Config};
use database::{connection::Connection, migrate, repository::Repository};
use router::{state::State, Router};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let db_connection = Connection::new(
        config.database.url.clone(),
        config.database.acquire_timeout,
        config.database.pool_size,
    )
    .connect()
    .await
    .expect("Cannot create connection pool");

    match config.command {
        Commands::Migrate => {
            migrate(&db_connection)
                .await
                .expect("Migration not successful");
        }
        Commands::Run(run_args) => {
            let state = State::new(Repository::new(db_connection));
            let router: AxumRouter = Router::new(run_args.root_path.into(), state).into();

            Server::bind(&run_args.bind_address)
                .serve(router.into_make_service())
                .await
                .expect("Cannot start server");
        }
    }
}
