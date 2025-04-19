use crate::error::{Error, Result};
use rusqlite::{Connection, Row};
use std::path::Path;

// 添加 Debug 特性实现
#[derive(Debug)]
pub struct RelationalStorage {
    conn: Connection,
}

impl RelationalStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        Ok(Self { conn })
    }
    
    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS layers (
                id TEXT PRIMARY KEY,
                document_id TEXT NOT NULL,
                name TEXT NOT NULL,
                visible INTEGER NOT NULL,
                locked INTEGER NOT NULL,
                opacity REAL NOT NULL,
                position INTEGER NOT NULL,
                FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE
            )",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        Ok(self.conn.execute(sql, params)?)
    }
    
    pub fn query_row<T, F>(&self, sql: &str, params: &[&dyn rusqlite::ToSql], f: F) -> Result<T>
    where
        F: FnOnce(&Row<'_>) -> rusqlite::Result<T>,
    {
        self.conn.query_row(sql, params, f)
            .map_err(|e| Error::Storage(format!("Query error: {}", e)))
    }
    
    pub fn query<T, F>(&self, sql: &str, params: &[&dyn rusqlite::ToSql], f: F) -> Result<Vec<T>>
    where
        F: FnMut(&Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, f)?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| Error::Storage(format!("Query error: {}", e)))?);
        }
        
        Ok(results)
    }
    
    // 修改参数从 &self 改为 &mut self
    pub fn transaction(&mut self) -> Result<rusqlite::Transaction> {
        Ok(self.conn.transaction()?)
    }
}