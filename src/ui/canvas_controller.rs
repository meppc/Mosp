
use std::sync::{Arc, Mutex};
use crate::canvas::Canvas;
use crate::tools::{ToolType, ToolContext, ToolEvent};
use crate::tools::manager::ToolManager;

pub struct CanvasController {
    canvas: Arc<Canvas>,
    tool_manager: Arc<Mutex<ToolManager>>,
    ui: slint::Weak<crate::ui::MainWindow>,
}

impl CanvasController {
    pub fn new(ui: slint::Weak<crate::ui::MainWindow>) -> Self {
        let canvas = Arc::new(Canvas::new("main".to_string(), "Main Canvas".to_string(), 1920, 1080));
        let tool_manager = Arc::new(Mutex::new(ToolManager::new()));
        
        Self {
            canvas,
            tool_manager,
            ui,
        }
    }
    
    pub fn setup_handlers(&self) {
    }
}
