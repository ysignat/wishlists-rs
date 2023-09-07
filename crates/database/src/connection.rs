use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::DataError;

pub struct DatabaseConnectOptions {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl From<DatabaseConnectOptions> for ConnectOptions {
    fn from(value: DatabaseConnectOptions) -> Self {
        let mut connection_opts = ConnectOptions::new(value.url);

        if let Some(max_connections) = value.max_connections {
            connection_opts.max_connections(max_connections);
        }

        if let Some(min_connections) = value.min_connections {
            connection_opts.min_connections(min_connections);
        }

        if let Some(connect_timeout) = value.connect_timeout {
            connection_opts.connect_timeout(connect_timeout);
        }

        if let Some(idle_timeout) = value.idle_timeout {
            connection_opts.idle_timeout(idle_timeout);
        }

        if let Some(acquire_timeout) = value.acquire_timeout {
            connection_opts.acquire_timeout(acquire_timeout);
        }

        if let Some(max_lifetime) = value.max_lifetime {
            connection_opts.max_lifetime(max_lifetime);
        }

        connection_opts
    }
}

pub struct Connection;

impl Connection {
    /// # Errors
    ///
    /// Will return `DataError` if database, specified in URL, is unreachable
    pub async fn connect(
        database_connect_options: DatabaseConnectOptions,
    ) -> Result<DatabaseConnection, DataError> {
        let connect_options: ConnectOptions = database_connect_options.into();
        Database::connect(connect_options)
            .await
            .or(Err(DataError::Unknown))
    }
}
