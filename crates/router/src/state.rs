use std::sync::Arc;

use database::Repository;

pub struct State {
    pub repository: Arc<dyn Repository + Send + Sync>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            repository: self.repository.clone(),
        }
    }
}
