
pub mod cache;
pub mod incremental;
pub mod relational;
pub mod migration;

use std::path::Path;
use crate::error::{Error, Result};

pub struct StorageManager {
}

impl StorageManager {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    pub fn initialize(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
