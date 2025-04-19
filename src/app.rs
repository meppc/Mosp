use crate::config::Config;
use crate::error::Result;
use crate::window::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> Result<Self> {
        let _config = Config::load()?;
        let window = Window::new()?;
        
        Ok(Self { window })
    }
    
    pub fn run(self) -> Result<()> {
        self.window.run()
    }
}
