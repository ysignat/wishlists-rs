use std::{net::SocketAddr, sync::Arc};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use database::{
    connection::get_db_connection,
    repository::Repository,
    repository_trait::RepositoryTrait,
};

use crate::config::Config;

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
    pub repository: Arc<dyn RepositoryTrait + Send + Sync>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            repository: self.repository.clone(),
        }
    }
}

pub fn get_root_path(root_path: &str) -> String {
    if root_path == "/" {
        String::new()
    } else {
        root_path.to_owned()
    }
}

pub async fn get_state(config: &Config) -> AppState {
    AppState {
        repository: Arc::new(Repository {
            database_connection: get_db_connection(
                &config.postgres_url,
                config.postgres_pool_acquire_timeout,
                config.postgres_pool_size,
            )
            .await,
        }),
    }
}

pub fn get_bind_address(bind_address: &str) -> SocketAddr {
    bind_address
        .parse()
        .unwrap_or_else(|_| panic!("Invalid bind address: {bind_address}"))
}
