
use crate::config::Config;
use crate::error::Result;
use crate::window::Window;
use slint::SharedString;

pub struct App {
    config: Config,
    window: Window,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let window = Window::new()?;
        
        Ok(Self {
            config,
            window,
        })
    }
    
    pub fn run(self) -> Result<()> {
        tracing::info!("Starting Mosp application");
        
        self.window.run()
    }
}
