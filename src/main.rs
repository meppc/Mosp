mod app;
mod canvas;
mod config;
mod error;
mod event;
mod storage;
mod tools;
mod window;
mod ui;

use app::App;
use slint::ComponentHandle;
use crate::ui::MainWindow;

fn main() {
    let app = App::new().expect("Failed to initialize application");
    
    let ui = MainWindow::new().unwrap();
    
    ui.on_exit(move || {
        std::process::exit(0);
    });
    
    ui.run().unwrap();
}
