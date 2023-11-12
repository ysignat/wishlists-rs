#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use axum::{Router as AxumRouter, Server};
use clap::Parser;
use config::{Commands, Config, LogFormat};
use database::{
    BlobStorageClient,
    BlobStorageConfig,
    Database,
    DatabaseConnectOptions,
    Repository,
};
use migrations::{Migrator, MigratorTrait};
use router::{state::State, Router};
use tracing::error;

mod config;
mod router;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let subscriber_builder = tracing_subscriber::fmt().with_max_level(config.log.level);

    if config.log.format == LogFormat::Json {
        subscriber_builder.json().init();
    } else {
        subscriber_builder.init();
    }

    let database_connect_options: DatabaseConnectOptions = config.database.into();
    let database_connection = Database::connect(database_connect_options)
        .await
        .unwrap_or_else(|_| {
            error!("Cannot create connection pool");
            panic!()
        });

    let blob_storage_config: BlobStorageConfig = config.blob_storage.into();
    let blob_storage_client = BlobStorageClient::from_conf(blob_storage_config);

    match config.command {
        Commands::Migrate => Migrator::up(&database_connection, None)
            .await
            .unwrap_or_else(|_| {
                error!("Migration not successful");
                panic!()
            }),
        Commands::Run(run_args) => {
            let state = State::new(Repository::new(database_connection, blob_storage_client));
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
