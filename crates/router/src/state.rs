use std::sync::Arc;

use database::repository::{Repository, RepositoryTrait};

pub struct State {
    pub repository: Arc<dyn RepositoryTrait + Send + Sync>,
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
