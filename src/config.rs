
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "Mosp".to_string(),
            window_width: 1280,
            window_height: 720,
            data_dir: default_data_dir(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }
    
    pub fn save(&self) -> Result<()> {
        Ok(())
    }
}

fn default_data_dir() -> PathBuf {
    let mut path = if let Some(proj_dirs) = directories::ProjectDirs::from("com", "mosp", "Mosp") {
        proj_dirs.data_dir().to_path_buf()
    } else {
        PathBuf::from("./data")
    };
    
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap_or_else(|_| {
            path = PathBuf::from("./data");
            std::fs::create_dir_all(&path).unwrap_or(());
        });
    }
    
    path
}
