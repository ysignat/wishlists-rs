use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DatabaseConnection;

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
    pub postgres_connection: DatabaseConnection,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            postgres_connection: self.postgres_connection.clone(),
        }
    }
}
