use std::sync::Arc;

use database::{
    ItemsRepository,
    Repository,
    SubscriptionsRepository,
    UsersRepository,
    WishlistsRepository,
};

pub struct State {
    pub(crate) repository: Arc<
        dyn ItemsRepository
            + SubscriptionsRepository
            + UsersRepository
            + WishlistsRepository
            + Send
            + Sync,
    >,
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            repository: self.repository.clone(),
        }
    }
}

impl State {
    #[must_use]
    pub fn new(database_repository: Repository) -> Self {
        State {
            repository: Arc::new(database_repository),
        }
    }
}
