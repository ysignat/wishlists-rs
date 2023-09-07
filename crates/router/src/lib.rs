#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use axum::Router as AxumRouter;
use handlers::{health, items, users, wishlists};
use state::State;

mod errors;
mod handlers;
pub mod state;

pub struct Router {
    root_path: String,
    state: State,
}

impl Router {
    #[must_use]
    pub fn new(root_path: String, state: State) -> Router {
        Router { root_path, state }
    }
}

impl From<Router> for AxumRouter {
    fn from(value: Router) -> Self {
        AxumRouter::new()
            .merge(users::get_router(&value.root_path, value.state.clone()))
            .merge(wishlists::get_router(&value.root_path, value.state.clone()))
            .merge(items::get_router(&value.root_path, value.state.clone()))
            .merge(health::get_router(&value.root_path, value.state.clone()))
    }
}
