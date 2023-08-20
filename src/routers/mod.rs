pub mod health;
pub mod items;
pub mod users;
pub mod wishlists;

use axum::Router as AxumRouter;

use crate::utils::AppState;

pub struct Router {
    root_path: String,
    state: AppState,
}

impl Router {
    pub fn new(root_path: String, state: AppState) -> Router {
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
