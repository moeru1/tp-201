#![deny(missing_docs)]
//! # KvStore 
//! 
//! A library to create a key-value store in memory

pub use anyhow::Result;
use std::{
    collections::HashMap,
    path::PathBuf,
    io::{BufReader, BufRead, Write},
    fs::File,
    fs,
};
use serde::{Serialize, Deserialize};
use LogCommands::{Set, Rm};



/// main struct of `KvStore`
pub struct KvStore {
    kv_memory_map: HashMap<String, String>,
    log_file: File,
}

/// Enum for the possible commands written to the log
#[derive(Serialize, Deserialize)]
enum LogCommands {
    Set(String, String),
    Rm(String),
}

impl KvStore {
    /// Creates an empty kv_store
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut kv_memory_map = HashMap::new();

        let path = path.into();
        let path = PathBuf::into_os_string(path.clone());
        if let Ok(log_file) = File::open(path.clone()) {

            let log_file_reader = BufReader::new(log_file);

            for line in log_file_reader.lines() {
                let line = line?;
                let command: LogCommands = serde_json::from_str(&line)?;
                match command {
                    Set(key, value) => {
                        kv_memory_map.insert(key, value);
                    },
                    Rm(key) => {
                        kv_memory_map.remove(&key);
                    }
                }
            }
        }
        
        let mut log_file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;


        Ok( KvStore { 
            kv_memory_map,
            log_file,
        })
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
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        let set_cmd = serde_json::to_string(&Set(key.clone(), value.clone()))?;
        self.log_file.write(set_cmd.as_bytes())?;
        self.kv_memory_map.insert(key, value);
        Ok(())
    }

    /// Gets the value associated with `key` if it exists 
    pub fn get(&self, key:String) -> Result<Option<String>> {
        Ok(self.kv_memory_map.get(&key).map(|x| x.to_owned()))
    }

    /// Removes a key from the key-value store, returning the value at the key 
    /// if the key was previously in the key-value store.
    pub fn remove(&mut self, key:String) -> Result<()> {
        let rm_cmd = serde_json::to_string(&Rm(key.clone()))?;
        self.log_file.write(rm_cmd.as_bytes())?;
        //send error if key doesnt exists
        self.kv_memory_map.remove(&key);
        Ok(())
    }
}
