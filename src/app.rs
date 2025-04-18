use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::storage::Storage;
use crate::window::Window;

pub struct App {
    config: Config,
    storage: Storage,
    window: Window,
}

impl App {
    pub fn new() -> Result<Self> {
        info!("Initializing application components...");
        
        let config = Config::load()?;
        let storage = Storage::new(&config)?;
        let window = Window::new(&config)?;

        Ok(Self {
            config,
            storage,
            window,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting application main loop...");
        
        // Initialize window and start event loop
        self.window.run().await?;

        Ok(())
    }
} 