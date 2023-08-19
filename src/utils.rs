use core::time::Duration;
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub struct AppState {
    pub database_connection: DatabaseConnection,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            database_connection: self.database_connection.clone(),
        }
    }
}

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

pub fn get_root_path(root_path: &str) -> String {
    if root_path == "/" {
        String::new()
    } else {
        root_path.to_owned()
    }
}

pub fn get_state(db_connection: DatabaseConnection) -> AppState {
    AppState {
        database_connection: db_connection,
    }
}

pub fn get_bind_address(bind_address: &str) -> SocketAddr {
    bind_address
        .parse()
        .unwrap_or_else(|_| panic!("Invalid bind address: {bind_address}"))
}
