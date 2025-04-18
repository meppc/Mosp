use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub storage: StorageConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub resizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub db_path: PathBuf,
    pub auto_save_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub default_tool: String,
    pub default_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1280,
                height: 720,
                title: "Mosp".to_string(),
                resizable: true,
            },
            storage: StorageConfig {
                db_path: PathBuf::from("data"),
                auto_save_interval: 300,
            },
            ui: UiConfig {
                theme: "light".to_string(),
                default_tool: "pen".to_string(),
                default_color: "#000000".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // TODO: Load from config file
        Ok(Self::default())
    }

    pub fn save(&self) -> Result<()> {
        // TODO: Save to config file
        Ok(())
    }
} 