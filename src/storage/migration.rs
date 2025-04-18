
use crate::error::Result;
use crate::storage::{IncrementalStorage, RelationalStorage};
use std::sync::Arc;

pub struct MigrationManager {
    incremental: Arc<IncrementalStorage>,
    relational: Arc<RelationalStorage>,
}

impl MigrationManager {
    pub fn new(
        incremental: Arc<IncrementalStorage>,
        relational: Arc<RelationalStorage>,
    ) -> Self {
        Self {
            incremental,
            relational,
        }
    }
    
    pub fn run_migrations(&self) -> Result<()> {
        self.relational.init_schema()?;
        
        let current_version = self.get_schema_version()?;
        
        match current_version {
            0 => {
                self.migrate_v0_to_v1()?;
                self.set_schema_version(1)?;
            }
            1 => {
            }
            _ => {
            }
        }
        
        Ok(())
    }
    
    fn get_schema_version(&self) -> Result<u32> {
        if let Some(version_data) = self.incremental.get("schema_version")? {
            let version_str = String::from_utf8_lossy(&version_data);
            Ok(version_str.parse().unwrap_or(0))
        } else {
            Ok(0)
        }
    }
    
    fn set_schema_version(&self, version: u32) -> Result<()> {
        self.incremental.put("schema_version", version.to_string().as_bytes())?;
        Ok(())
    }
    
    fn migrate_v0_to_v1(&self) -> Result<()> {
        self.relational.init_schema()?;
        
        Ok(())
    }
}
