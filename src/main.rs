mod app;
mod canvas;
mod config;
mod error;
mod event;
mod i18n;
mod storage;
mod tools;
mod window;
mod ui;

use app::App;
use slint::ComponentHandle;

fn main() {
    let mut app = App::new().expect("Failed to initialize application");
    
    let ui = ui::MainWindow::new().unwrap();
    
    ui::setup_localization(&ui);
    
    app.set_ui(ui.as_weak());
    
    ui.on_exit(move || {
        std::process::exit(0);
    });
    
    ui.run().unwrap();
}
