use std::sync::Arc;

use database::repository::DatabaseRepositoryTrait;

pub struct State {
    pub repository: Arc<dyn DatabaseRepositoryTrait + Send + Sync>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            repository: self.repository.clone(),
        }
    }
}
