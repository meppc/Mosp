use crate::error::Result;
use slint::{PhysicalSize, WindowSize};

slint::include_modules!();

pub struct Window {
    ui: MainWindow,
}

impl Window {
    pub fn new() -> Result<Self> {
        let ui = MainWindow::new()?;
        
        // 修复 set_title 方法，根据错误信息，可能应该使用 set_window_title
        // 或者完全移除这行代码
        // ui.window().set_title("Mosp");
        ui.window().set_size(WindowSize::Physical(PhysicalSize::new(1280, 720)));
        
        Self::setup_callbacks(&ui);
        
        Ok(Self { ui })
    }
    
    pub fn run(self) -> Result<()> {
        self.ui.run()?;
        Ok(())
    }
    
    fn setup_callbacks(ui: &MainWindow) {
        // 添加下划线前缀避免未使用变量警告
        let _ui_handle = ui.clone_strong();
        ui.on_exit(move || {
            slint::quit_event_loop().unwrap();
        });
    }
}