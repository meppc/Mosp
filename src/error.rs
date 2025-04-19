use thiserror::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Canvas error: {0}")]
    Canvas(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("{0}")]
    Generic(String),
}

impl Error {
    pub fn canvas<T: ToString>(msg: T) -> Self {
        Error::Canvas(msg.to_string())
    }
    
    pub fn storage<T: ToString>(msg: T) -> Self {
        Error::Storage(msg.to_string())
    }
    
    pub fn generic<T: ToString>(msg: T) -> Self {
        Error::Generic(msg.to_string())
    }
}
