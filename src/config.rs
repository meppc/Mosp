use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub data_dir: PathBuf,
    pub theme: String,
    pub auto_save: bool,
    pub auto_save_interval: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "Mosp".to_string(),
            window_width: 1280,
            window_height: 720,
            data_dir: default_data_dir(),
            theme: "light".to_string(),
            auto_save: true,
            auto_save_interval: 300, // 5 minutes
        }
    }
}

impl Config {
    /// 从配置文件加载配置，如果文件不存在则创建默认配置
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            info!("Config file not found, creating default config");
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let config_str = fs::read_to_string(&config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let config_str = serde_json::to_string_pretty(self)?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&config_path, config_str)?;
        info!("Configuration saved to {:?}", config_path);
        
        Ok(())
    }
    
    /// 获取配置文件路径
    fn config_path() -> Result<PathBuf> {
        let mut path = if let Some(proj_dirs) = directories::ProjectDirs::from("com", "mosp", "Mosp") {
            proj_dirs.config_dir().to_path_buf()
        } else {
            PathBuf::from("./config")
        };
        
        path.push("config.json");
        Ok(path)
    }
    
    /// 验证配置是否有效
    pub fn validate(&self) -> Result<()> {
        if self.window_width < 800 || self.window_height < 600 {
            return Err(crate::error::Error::Config(
                "Window dimensions too small".to_string()
            ));
        }
        
        if !["light", "dark"].contains(&self.theme.as_str()) {
            return Err(crate::error::Error::Config(
                "Invalid theme".to_string()
            ));
        }
        
        if self.auto_save_interval < 60 {
            return Err(crate::error::Error::Config(
                "Auto-save interval too short".to_string()
            ));
        }
        
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
        if let Err(e) = fs::create_dir_all(&path) {
            error!("Failed to create data directory: {}", e);
            path = PathBuf::from("./data");
            fs::create_dir_all(&path).unwrap_or(());
        }
    }
    
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.app_name, "Mosp");
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
        assert_eq!(config.theme, "light");
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 300);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // 测试有效的配置
        assert!(config.validate().is_ok());
        
        // 测试窗口尺寸过小
        config.window_width = 600;
        assert!(config.validate().is_err());
        
        // 测试无效的主题
        config.window_width = 1280;
        config.theme = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // 测试自动保存间隔过短
        config.theme = "light".to_string();
        config.auto_save_interval = 30;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // 创建临时配置
        let mut config = Config::default();
        config.data_dir = temp_dir.path().to_path_buf();
        
        // 保存配置
        let config_str = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_str).unwrap();
        
        // 加载配置
        let config_str = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&config_str).unwrap();
        
        assert_eq!(config.app_name, loaded_config.app_name);
        assert_eq!(config.window_width, loaded_config.window_width);
        assert_eq!(config.window_height, loaded_config.window_height);
        assert_eq!(config.theme, loaded_config.theme);
        assert_eq!(config.auto_save, loaded_config.auto_save);
        assert_eq!(config.auto_save_interval, loaded_config.auto_save_interval);
    }
}
