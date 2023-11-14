use std::sync::Arc;

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct KeyValueStore {
    store: Arc<DashMap<String, String>>
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore { store: Arc::new(DashMap::new()) }
    }

    pub fn put(self, key: String, value: String) -> Result<(String, String), KeyNotFoundError> {
        self.store.insert(key.clone(), value.clone()); // this operation replaces the prexisting value if there was one
        Ok((key, value))
    }

    pub fn get(self, key: String) -> Result<(String, String), KeyNotFoundError> {
        if let Some(value) = self.store.get(&key) {
            Ok((key, value.clone()))
        } else { 
            Err(KeyNotFoundError {key})
        }
    }

    pub fn delete(self, key: String) -> Result<(String, String), KeyNotFoundError> {
        if let Some(key_value_pair) = self.store.remove(&key) {
            Ok(key_value_pair)
        } else {
            Err(KeyNotFoundError {key})
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyNotFoundError {
    key: String
}

