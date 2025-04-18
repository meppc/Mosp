
use crate::error::{Error, Result};
use std::path::Path;

pub struct IncrementalStorage {
    db: sled::Db,
}

impl IncrementalStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
    
    pub fn put(&self, key: &str, value: &[u8]) -> Result<()> {
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let result = self.db.get(key.as_bytes())?;
        Ok(result.map(|ivec| ivec.to_vec()))
    }
    
    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn contains(&self, key: &str) -> Result<bool> {
        Ok(self.db.contains_key(key.as_bytes())?)
    }
    
    pub fn scan_prefix(&self, prefix: &str) -> Vec<String> {
        let prefix_bytes = prefix.as_bytes();
        self.db
            .scan_prefix(prefix_bytes)
            .filter_map(|res| {
                res.ok().map(|(key, _)| {
                    String::from_utf8_lossy(&key).to_string()
                })
            })
            .collect()
    }
    
    pub fn open_tree(&self, name: &str) -> Result<sled::Tree> {
        Ok(self.db.open_tree(name)?)
    }
}
