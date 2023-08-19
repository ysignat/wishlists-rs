use axum::Router;

use crate::utils::AppState;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

static SUBPATH: &str = "/users";

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{root_path}{SUBPATH}"),
            axum::routing::get(list::handler).post(create::handler),
        )
        .route(
            &format!("{root_path}{SUBPATH}/:id"),
            axum::routing::get(get::handler)
                .put(update::handler)
                .delete(delete::handler),
        )
        .with_state(state)
}
