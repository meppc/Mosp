use anyhow::Result;
use sled::Db;
use rusqlite::Connection;

use crate::config::Config;

pub struct Storage {
    db: Db,
    sql: Connection,
}

impl Storage {
    pub fn new(config: &Config) -> Result<Self> {
        // Ensure the data directory exists
        std::fs::create_dir_all(&config.storage.db_path)?;

        // Initialize sled database
        let db_path = config.storage.db_path.join("data.sled");
        let db = sled::open(db_path)?;

        // Initialize SQLite database
        let sql_path = config.storage.db_path.join("data.sqlite");
        let sql = Connection::open(sql_path)?;

        // Initialize database schema
        Self::init_schema(&sql)?;

        Ok(Self { db, sql })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS strokes (
                id TEXT PRIMARY KEY,
                document_id TEXT NOT NULL,
                data BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (document_id) REFERENCES documents(id)
            )",
            [],
        )?;

        Ok(())
    }
} 