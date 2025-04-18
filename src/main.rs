
mod app;
mod config;
mod error;
mod storage;
mod window;
mod ui;

use crate::app::App;
use crate::error::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let app = App::new()?;
    app.run()
}
