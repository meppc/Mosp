
use slint::Weak;

pub struct WindowManager {
    main_window: Weak<crate::ui::MainWindow>,
}

impl WindowManager {
    pub fn new(main_window: Weak<crate::ui::MainWindow>) -> Self {
        Self { main_window }
    }
    
    pub fn show_error(&self, message: &str) {
        if let Some(ui) = self.main_window.upgrade() {
            ui.invoke_show_error(slint::SharedString::from(message));
        }
    }
    
    pub fn show_info(&self, message: &str) {
        if let Some(ui) = self.main_window.upgrade() {
            ui.invoke_show_info(slint::SharedString::from(message));
        }
    }
}
