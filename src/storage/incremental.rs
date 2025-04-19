
use std::path::Path;
use crate::error::Result;

pub struct IncrementalStorage {
}

impl IncrementalStorage {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    pub fn initialize(&self, path: &Path) -> Result<()> {
        Ok(())
    }
}
