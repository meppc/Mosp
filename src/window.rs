
use crate::error::Result;
use slint::{PhysicalSize, WindowSize};

slint::include_modules!();

pub struct Window {
    ui: MainWindow,
}

impl Window {
    pub fn new() -> Result<Self> {
        let ui = MainWindow::new()?;
        
        ui.window().set_title("Mosp");
        ui.window().set_size(WindowSize::Physical(PhysicalSize::new(1280, 720)));
        
        Self::setup_callbacks(&ui);
        
        Ok(Self { ui })
    }
    
    pub fn run(self) -> Result<()> {
        self.ui.run()?;
        Ok(())
    }
    
    fn setup_callbacks(ui: &MainWindow) {
        let ui_handle = ui.clone_strong();
        ui.on_exit(move || {
            slint::quit_event_loop().unwrap();
        });
    }
}
