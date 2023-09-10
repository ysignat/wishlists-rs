#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
mod config;

use axum::{Router as AxumRouter, Server};
use clap::Parser;
use config::{Commands, Config, LogFormat};
use database::{ConnectOptions, Database, Migrator, MigratorTrait, Repository};
use router::{state::State, Router};
use tracing::error;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let subscriber_builder = tracing_subscriber::fmt().with_max_level(config.log.level);

    if config.log.format == LogFormat::Json {
        subscriber_builder.json().init();
    } else {
        subscriber_builder.init();
    }

    let db_connect_options: ConnectOptions = config.database.into();
    let db_connection = Database::connect(db_connect_options)
        .await
        .unwrap_or_else(|_| {
            error!("Cannot create connection pool");
            panic!()
        });

    match config.command {
        Commands::Migrate => Migrator::up(&db_connection, None)
            .await
            .unwrap_or_else(|_| {
                error!("Migration not successful");
                panic!()
            }),
        Commands::Run(run_args) => {
            let state = State::new(Repository::new(db_connection));
            let router: AxumRouter = Router::new(run_args.root_path.into(), state).into();

            Server::bind(&run_args.bind_address)
                .serve(router.into_make_service())
                .await
                .unwrap_or_else(|_| {
                    error!("Cannot start server ({})", run_args.bind_address);
                    panic!()
                });
        }
    }
}
