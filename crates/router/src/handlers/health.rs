use axum::{extract::State as AxumState, http::StatusCode, Router};

use crate::state::State;

static SUBPATH: &str = "/health";

pub fn get_router(root_path: &str, state: State) -> Router {
    Router::new().route(
        &format!("{root_path}{SUBPATH}"),
        axum::routing::get(healthcheck).with_state(state),
    )
}

#[allow(clippy::unused_async)]
async fn healthcheck(AxumState(state): AxumState<State>) -> StatusCode {
    match state.repository.healthcheck().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
