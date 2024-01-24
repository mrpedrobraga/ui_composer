use std::error::Error;

mod logging;
mod program;
mod renderer;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup_logger();
    program::run().await?;
    Ok(())
}