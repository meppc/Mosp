
use std::path::PathBuf;
use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load().unwrap_or_else(|_| Config::default())
});

#[derive(Debug, Clone)]
pub struct Config {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub default_canvas_width: u32,
    pub default_canvas_height: u32,
}

impl Config {
    pub fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "mosp", "Mosp")
            .expect("Failed to determine project directories");
        
        Self {
            config_dir: proj_dirs.config_dir().to_path_buf(),
            data_dir: proj_dirs.data_dir().to_path_buf(),
            cache_dir: proj_dirs.cache_dir().to_path_buf(),
            default_canvas_width: 1920,
            default_canvas_height: 1080,
        }
    }
    
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::default())
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
