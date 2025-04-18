
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct StorageManager {
    incremental_db: sled::Db,
    relational_db: rusqlite::Connection,
}

impl StorageManager {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref();
        std::fs::create_dir_all(data_dir)?;
        
        let incremental_path = data_dir.join("incremental");
        let incremental_db = sled::open(incremental_path)?;
        
        let relational_path = data_dir.join("relational.db");
        let relational_db = rusqlite::Connection::open(relational_path)?;
        
        Self::init_schema(&relational_db)?;
        
        Ok(Self {
            incremental_db,
            relational_db,
        })
    }
    
    fn init_schema(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| Error::Storage(format!("Serialization error: {}", e)))?;
        
        self.incremental_db
            .insert(key.as_bytes(), serialized)?;
        
        Ok(())
    }
    
    pub fn retrieve<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        if let Some(data) = self.incremental_db.get(key.as_bytes())? {
            let deserialized = serde_json::from_slice(&data)
                .map_err(|e| Error::Storage(format!("Deserialization error: {}", e)))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }
}
