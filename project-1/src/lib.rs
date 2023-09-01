#![deny(missing_docs)]
//! # KvStore 
//! 
//! A library to create a key-value store in memory

use std::collections::HashMap;

/// main struct of `KvStore`
pub struct KvStore {
    kv_map: HashMap<String, String>
}

impl KvStore {
    /// Creates an empty KvStore
    pub fn new() -> Self {
        KvStore {kv_map: HashMap::new()}
    }

    /// Sets a key-value pair into the `KvStore`
    /// # Examples 
    /// ```
    /// let mut kv_store = kvs::KvStore::new();
    /// kv_store.set("key1".to_owned(), "value1".to_owned());
    /// kv_store.set("key1".to_owned(), "new_value".to_owned());
    /// assert_eq!(kv_store.get("key1".to_owned()), Some("new_value".to_owned()));
    /// assert_eq!(kv_store.get("key2".to_owned()), None);
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Option<String>{
        self.kv_map.insert(key, value)
    }

    /// Gets the value associated with `key` if it exists 
    pub fn get(&self, key:String) -> Option<String> {
        let val = self.kv_map.get(&key).map(|x| x.to_owned());
        val
    }

    /// Removes a key from the key-value store, returning the value at the key 
    /// if the key was previously in the key-value store.
    pub fn remove(&mut self, key:String) -> Option<String> {
        self.kv_map.remove(&key)
    }
}
