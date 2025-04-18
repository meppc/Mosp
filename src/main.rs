use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod app;
mod config;
mod error;
mod storage;
mod ui;
mod window;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_thread_names(true)
        .with_ansi(true)
        .pretty()
        .init();

    info!("Starting Mosp application...");

    // Initialize application
    let app = app::App::new()?;
    app.run().await?;

    Ok(())
}