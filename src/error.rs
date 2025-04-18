
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("UI error: {0}")]
    Ui(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("{0}")]
    Generic(String),
}

impl From<slint::PlatformError> for Error {
    fn from(err: slint::PlatformError) -> Self {
        Self::Ui(err.to_string())
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Self::Storage(err.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Storage(err.to_string())
    }
}
