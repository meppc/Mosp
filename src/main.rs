
mod app;
mod canvas;
mod config;
mod error;
mod event;
mod storage;
mod window;
mod ui;

use crate::app::App;
use crate::error::Result;
use crate::event::{global_event_bus, events::AppStartupEvent, events::AppShutdownEvent};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Mosp application starting");
    
    global_event_bus().dispatch(AppStartupEvent);
    
    let app = App::new()?;
    let result = app.run();
    
    global_event_bus().dispatch(AppShutdownEvent);
    
    result
}
