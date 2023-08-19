use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::errors::DataError;

pub struct Connection {
    url: String,
    acquire_timeout: u64,
    max_connections: u32,
}

impl Connection {
    #[must_use]
    pub fn new(url: String, acquire_timeout: u64, max_connections: u32) -> Self {
        Connection {
            url,
            acquire_timeout,
            max_connections,
        }
    }

    /// # Errors
    ///
    /// Will return `DataError` if database, specified in URL, is unreachable
    pub async fn connect(&self) -> Result<DatabaseConnection, DataError> {
        let mut connection_opts = ConnectOptions::new(self.url.clone());
        connection_opts
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .max_connections(self.max_connections);

        Database::connect(connection_opts)
            .await
            .or(Err(DataError::Unknown))
    }
}
