use anyhow::Result;
use slint::{CloseRequestResponse, ComponentHandle, PhysicalPosition, PhysicalSize, WindowPosition, WindowSize};
use tracing::info;

use crate::config::Config;
use crate::ui::MainWindow;

pub struct Window {
    window: MainWindow,
}

impl Window {
    pub fn new(config: &Config) -> Result<Self> {
        info!("Initializing main window...");

        let window = MainWindow::new()?;
        
        // Configure window properties
        let slint_window = window.window();
        slint_window.set_size(WindowSize::Physical(
            PhysicalSize::new(config.window.width, config.window.height)
        ));
        
        slint_window.set_position(WindowPosition::Physical(
            PhysicalPosition::new(100, 100)
        ));

        Ok(Self { window })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting window event loop...");
        
        // Set up event handlers
        self.setup_event_handlers();
        
        // Run the main event loop
        self.window.run()?;
        
        Ok(())
    }

    fn setup_event_handlers(&self) {
        let window = self.window.as_weak();
        
        // Handle window close
        self.window.window().on_close_requested(move || {
            if let Some(window) = window.upgrade() {
                window.hide().unwrap();
                CloseRequestResponse::HideWindow
            } else {
                CloseRequestResponse::HideWindow
            }
        });
        
        // TODO: Add more event handlers
    }
} 