use color_eyre::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use ukodus::gui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let log_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(log_subscriber)?;

    info!("Launching GUI app!");
    gui::app::run()?;
    info!("Exited GUI app!");

    Ok(())
}
