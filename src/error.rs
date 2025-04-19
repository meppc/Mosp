use thiserror::Error;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// 配置相关错误
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// 文件系统错误
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    /// 序列化/反序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// 数据库错误
    #[error("Database error: {0}")]
    Database(String),
    
    /// 事件处理错误
    #[error("Event handling error: {0}")]
    Event(String),
    
    /// 窗口管理错误
    #[error("Window management error: {0}")]
    Window(String),
    
    /// 画布操作错误
    #[error("Canvas operation error: {0}")]
    Canvas(String),
    
    /// 存储错误
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// 通用错误
    #[error("Error: {0}")]
    Generic(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// 创建新的配置错误
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    /// 创建新的数据库错误
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }
    
    /// 创建新的事件处理错误
    pub fn event(msg: impl Into<String>) -> Self {
        Self::Event(msg.into())
    }
    
    /// 创建新的窗口管理错误
    pub fn window(msg: impl Into<String>) -> Self {
        Self::Window(msg.into())
    }
    
    /// 创建新的画布操作错误
    pub fn canvas(msg: impl Into<String>) -> Self {
        Self::Canvas(msg.into())
    }
    
    /// 创建新的存储错误
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }
    
    /// 创建新的通用错误
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Self::Storage(err.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<slint::PlatformError> for Error {
    fn from(err: slint::PlatformError) -> Self {
        Self::Window(err.to_string())
    }
}
