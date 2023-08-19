use axum::{http::StatusCode, Router};

use crate::utils::AppError;

static SUBPATH: &str = "/health";

pub fn get_router(root_path: &str) -> Router {
    Router::new().route(&format!("{root_path}{SUBPATH}"), axum::routing::get(health))
}

#[allow(clippy::unused_async)]
async fn health() -> Result<(StatusCode, String), AppError> {
    Ok((StatusCode::OK, "Healthy!".to_owned()))
}
