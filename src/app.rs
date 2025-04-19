use crate::error::Result;
use std::sync::Arc;

pub struct App {
    ui: Option<slint::Weak<crate::ui::MainWindow>>,
    canvas_controller: Option<Arc<crate::ui::canvas_controller::CanvasController>>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ui: None,
            canvas_controller: None,
        })
    }
    
    pub fn set_ui(&mut self, ui: slint::Weak<crate::ui::MainWindow>) {
        let controller = Arc::new(crate::ui::canvas_controller::CanvasController::new(ui.clone()));
        controller.setup_handlers();
        
        self.ui = Some(ui);
        self.canvas_controller = Some(controller);
    }
}
