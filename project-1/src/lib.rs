use std::{process::exit, collections::HashMap};

pub struct KvStore {
    kv_map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {kv_map: HashMap::new()}
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String>{
        self.kv_map.insert(key, value)
    }

    pub fn get(&self, key:String) -> Option<String> {
        let val = self.kv_map.get(&key).map(|x| x.to_owned());
        val
    }

    pub fn remove(&mut self, key:String) -> Option<String> {
        self.kv_map.remove(&key)
    }
}
