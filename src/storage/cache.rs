
use crate::error::Result;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Mutex;

pub struct Cache {
    memory_cache: Mutex<LruCache<String, Vec<u8>>>,
    disk_cache: sled::Db,
}

impl Cache {
    pub fn new<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self> {
        let memory_cache = Mutex::new(LruCache::new(NonZeroUsize::new(capacity).unwrap()));
        
        let disk_cache = sled::open(path)?;
        
        Ok(Self {
            memory_cache,
            disk_cache,
        })
    }
    
    pub fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.put(key.to_string(), value.to_vec());
        
        self.disk_cache.insert(key.as_bytes(), value)?;
        
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        if let Some(value) = memory_cache.get(key) {
            return Ok(Some(value.clone()));
        }
        
        if let Some(value) = self.disk_cache.get(key.as_bytes())? {
            let value_vec = value.to_vec();
            
            memory_cache.put(key.to_string(), value_vec.clone());
            
            return Ok(Some(value_vec));
        }
        
        Ok(None)
    }
    
    pub fn delete(&self, key: &str) -> Result<()> {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.pop(key);
        
        self.disk_cache.remove(key.as_bytes())?;
        
        Ok(())
    }
    
    pub fn clear(&self) -> Result<()> {
        let mut memory_cache = self.memory_cache.lock().unwrap();
        memory_cache.clear();
        
        self.disk_cache.clear()?;
        
        Ok(())
    }
}
