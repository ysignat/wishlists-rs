use axum::Router;

use crate::state::State;

mod create;
mod delete;
mod get;
mod list;
mod update;

static SUBPATH: &str = "/items";

pub fn get_router(root_path: &str, state: State) -> Router {
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
