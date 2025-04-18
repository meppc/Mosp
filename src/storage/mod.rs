
mod incremental;
mod relational;
mod cache;
mod migration;

pub use incremental::IncrementalStorage;
pub use relational::RelationalStorage;
pub use cache::Cache;
pub use migration::MigrationManager;

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum StorageType {
    Incremental(Arc<IncrementalStorage>),
    Relational(Arc<RelationalStorage>),
}

pub trait Serializable: Sized {
    fn serialize(&self) -> Result<Vec<u8>>;
    
    fn deserialize(data: &[u8]) -> Result<Self>;
}

pub struct StorageManager {
    incremental: Arc<IncrementalStorage>,
    relational: Arc<RelationalStorage>,
    cache: Arc<Cache>,
}

impl StorageManager {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref();
        std::fs::create_dir_all(data_dir)?;
        
        let incremental = Arc::new(IncrementalStorage::new(data_dir.join("incremental"))?);
        
        let relational = Arc::new(RelationalStorage::new(data_dir.join("relational.db"))?);
        
        let cache = Arc::new(Cache::new(data_dir.join("cache"), 1000)?);
        
        let migration_manager = MigrationManager::new(
            incremental.clone(),
            relational.clone(),
        );
        migration_manager.run_migrations()?;
        
        Ok(Self {
            incremental,
            relational,
            cache,
        })
    }
    
    pub fn incremental(&self) -> Arc<IncrementalStorage> {
        self.incremental.clone()
    }
    
    pub fn relational(&self) -> Arc<RelationalStorage> {
        self.relational.clone()
    }
    
    pub fn cache(&self) -> Arc<Cache> {
        self.cache.clone()
    }
    
    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| Error::Storage(format!("Serialization error: {}", e)))?;
        
        self.cache.set(key, &serialized)?;
        
        self.incremental.put(key, &serialized)?;
        
        Ok(())
    }
    
    pub fn retrieve<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        if let Some(data) = self.cache.get(key)? {
            let deserialized = serde_json::from_slice(&data)
                .map_err(|e| Error::Storage(format!("Deserialization error: {}", e)))?;
            return Ok(Some(deserialized));
        }
        
        if let Some(data) = self.incremental.get(key)? {
            self.cache.set(key, &data)?;
            
            let deserialized = serde_json::from_slice(&data)
                .map_err(|e| Error::Storage(format!("Deserialization error: {}", e)))?;
            return Ok(Some(deserialized));
        }
        
        Ok(None)
    }
    
    pub fn delete(&self, key: &str) -> Result<()> {
        self.cache.delete(key)?;
        
        self.incremental.delete(key)?;
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasData {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<LayerData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerData {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
}

impl Serializable for CanvasData {
    fn serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| Error::Storage(format!("Serialization error: {}", e)))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| Error::Storage(format!("Deserialization error: {}", e)))
    }
}
