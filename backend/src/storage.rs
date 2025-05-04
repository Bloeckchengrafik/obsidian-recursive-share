pub mod context;
pub mod meta;

use std::path::PathBuf;
use crate::config::Config;
use crate::storage::context::StorageContext;

pub struct Storage{
    path: PathBuf,
}

impl Storage {
    pub async fn new(config: &Config) -> Self {
        Self {
            path: PathBuf::from(config.get_storage_path().await)
        }
    }

    pub fn allocate_new(&self, root_name: &str) -> StorageContext {
        let path = uuid::Uuid::new_v4().to_string();
        StorageContext::new(self.path.clone(), &path, root_name)
    }

    pub fn get_existing(&self, id: &str) -> Option<StorageContext> {
        let path = self.path.join(id);
        if path.exists() {
            Some(StorageContext::load(path))
        } else {
            None
        }
    }
}
