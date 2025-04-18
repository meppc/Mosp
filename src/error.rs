use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Window error: {0}")]
    Window(String),

    #[error("UI error: {0}")]
    Ui(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("File system error: {0}")]
    FileSystem(String),
}

pub type Result<T> = std::result::Result<T, Error>; 