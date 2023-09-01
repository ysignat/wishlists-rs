use std::{net::SocketAddr, sync::Arc};

use database::{connection::Connection, DatabaseRepository};
use router::state::State;

use crate::config::Config;

pub fn get_root_path(root_path: &str) -> String {
    if root_path == "/" {
        String::new()
    } else {
        root_path.to_owned()
    }
}

pub async fn get_state(config: &Config) -> Result<State, database::DataError> {
    Ok(State {
        repository: Arc::new(DatabaseRepository {
            database_connection: Connection::new(
                config.postgres_url.clone(),
                config.postgres_pool_acquire_timeout,
                config.postgres_pool_size,
            )
            .connect()
            .await?,
        }),
    })
}

pub fn get_bind_address(bind_address: &str) -> SocketAddr {
    bind_address
        .parse()
        .unwrap_or_else(|_| panic!("Invalid bind address: {bind_address}"))
}
