use crate::error::Result;
use std::path::Path;

#[derive(Debug)]
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
        Ok(result.map(|v| v.to_vec()))
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
        self.db
            .scan_prefix(prefix.as_bytes())
            .filter_map(|item| {
                item.ok().and_then(|(key, _)| {
                    String::from_utf8(key.to_vec()).ok()
                })
            })
            .collect()
    }
    
    pub fn open_tree(&self, name: &str) -> Result<sled::Tree> {
        Ok(self.db.open_tree(name)?)
    }
}