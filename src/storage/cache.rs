
use std::path::Path;
use crate::error::Result;

pub struct CacheManager {
}

impl CacheManager {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    pub fn initialize(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
