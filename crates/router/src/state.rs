use std::sync::Arc;

use database::repository_trait::RepositoryTrait;

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
