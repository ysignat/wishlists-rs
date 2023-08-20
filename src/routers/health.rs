use axum::{extract::State, http::StatusCode, Router};

use crate::utils::AppState;

static SUBPATH: &str = "/health";

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new().route(
        &format!("{root_path}{SUBPATH}"),
        axum::routing::get(healthcheck).with_state(state),
    )
}

#[allow(clippy::unused_async)]
async fn healthcheck(State(state): State<AppState>) -> StatusCode {
    match state.repository.healthcheck().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
