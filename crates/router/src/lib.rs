#![warn(clippy::pedantic)]
use axum::Router as AxumRouter;
use handlers::{health, items, users, wishlists};
use state::State;

pub mod errors;
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

    pub fn build(&self) -> AxumRouter {
        AxumRouter::new()
            .merge(users::get_router(&self.root_path, self.state.clone()))
            .merge(wishlists::get_router(&self.root_path, self.state.clone()))
            .merge(items::get_router(&self.root_path, self.state.clone()))
            .merge(health::get_router(&self.root_path, self.state.clone()))
    }
}
