#![deny(missing_docs)]
//! # KvStore 
//! 
//! A library to create a key-value store in memory

pub use anyhow::Result;
use anyhow::anyhow;
use std::{
    collections::HashMap,
    path::PathBuf,
    io::{BufReader, BufRead, Write, Seek, SeekFrom},
    fs::{File, self},
    fmt,
};
use serde::{Serialize, Deserialize};
use LogCommands::{Set, Rm};


/// Error type 
#[derive(Debug, Clone)]
pub struct KeyNotFound;

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


impl fmt::Display for KeyNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key not Found")
    }
}


impl KvStore {
    /// Creates an empty kv_store
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let log_file_name = "kv_store.log";
        let index_file_name = "index.idx";

        let mut kv_memory_map = HashMap::new();

        let path = path.into();
        //let path = PathBuf::into_boxed_path(path.clone());
        if !path.is_dir() {
            return Err(anyhow!("{:?} needs to be a directory!", path));
        }


        /*Replay log file***********************************************/
        let mut path_log_file = path.clone();
        path_log_file.push(log_file_name);
        
        if let Ok(log_file) = File::open(path_log_file.clone()) {
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
        /************************************************************/

        let log_file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path_log_file)?;


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
        write!(&self.log_file, "{}\n", set_cmd)?;
        //self.log_file.write(set_cmd.as_bytes())?;
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
        //self.log_file.write(rm_cmd.as_bytes())?;
        //send error if key doesnt exists
        match self.kv_memory_map.remove(&key) {
            None => Err(anyhow!(KeyNotFound)),
            Some(_) => {
                write!(&self.log_file, "{}\n", rm_cmd)?;
                Ok(())
            }
        }
    }
}
