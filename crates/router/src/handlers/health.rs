use axum::{http::StatusCode, Router};

use crate::state::State;

static SUBPATH: &str = "/health";

pub(crate) fn get_router(root_path: &str, state: State) -> Router {
    Router::new().route(
        &format!("{root_path}{SUBPATH}"),
        axum::routing::get(healthcheck).with_state(state),
    )
}

#[allow(clippy::unused_async)]
async fn healthcheck() -> StatusCode {
    // match state.repository.healthcheck().await {
    //     Ok(_) => StatusCode::OK,
    //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    // }
    StatusCode::OK
}
