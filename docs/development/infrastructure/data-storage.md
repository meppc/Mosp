# Data Storage Implementation

## Overview
This document details the implementation of the data storage system for Mosp, including the integration of sled for incremental storage and SQLite for relational data.

## Storage Architecture

### Data Model
```rust
pub enum StorageType {
    Incremental(sled::Db),
    Relational(sqlx::SqlitePool),
}

pub struct StorageManager {
    storage: StorageType,
    cache: Cache,
}
```

### Serialization
```rust
pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
    fn deserialize(data: &[u8]) -> Result<Self, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct CanvasData {
    layers: Vec<LayerData>,
    metadata: Metadata,
}
```

## Incremental Storage (sled)

### Implementation
```rust
pub struct IncrementalStorage {
    db: sled::Db,
    tree: sled::Tree,
}

impl IncrementalStorage {
    pub fn new(path: &Path) -> Result<Self, Error> {
        // Implementation
    }
    
    pub fn save(&self, key: &str, data: &[u8]) -> Result<(), Error> {
        // Implementation
    }
}
```

### Key Structure
- `canvas/{id}/layers`
- `canvas/{id}/metadata`
- `user/{id}/preferences`

## Relational Storage (SQLite)

### Schema
```sql
CREATE TABLE canvases (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE layers (
    id TEXT PRIMARY KEY,
    canvas_id TEXT NOT NULL,
    name TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases(id)
);
```

### Implementation
```rust
pub struct RelationalStorage {
    pool: sqlx::SqlitePool,
}

impl RelationalStorage {
    pub async fn new(path: &Path) -> Result<Self, Error> {
        // Implementation
    }
    
    pub async fn save_canvas(&self, canvas: &Canvas) -> Result<(), Error> {
        // Implementation
    }
}
```

## Caching System

### Implementation
```rust
pub struct Cache {
    memory: LruCache<String, Vec<u8>>,
    disk: sled::Db,
}

impl Cache {
    pub fn new(size: usize) -> Self {
        // Implementation
    }
    
    pub fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        // Implementation
    }
}
```

## Data Migration

### Version Control
```rust
pub struct MigrationManager {
    current_version: u32,
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationManager {
    pub fn migrate(&self) -> Result<(), Error> {
        // Implementation
    }
}
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_incremental_storage() {
        // Test implementation
    }
    
    #[test]
    fn test_relational_storage() {
        // Test implementation
    }
}
```

### Integration Tests
- Data consistency tests
- Performance benchmarks
- Migration tests

## Related Documents
- [Project Setup](./project-setup.md)
- [Canvas Implementation](./canvas-implementation.md)
- [Version Control](../export/version-control.md) 