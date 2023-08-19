use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_db_connection(
    url: &str,
    acquire_timeout: u64,
    max_connections: u32,
) -> DatabaseConnection {
    let mut connection_opts = ConnectOptions::new(url.to_owned());
    connection_opts
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .max_connections(max_connections);

    Database::connect(connection_opts)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Cannot create connection pool with following params: URL - '{url}', acquire_timeout - '{acquire_timeout}', max_connections - '{max_connections}'"
            )
        })
}
