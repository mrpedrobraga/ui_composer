use std::error::Error;

mod logging;
mod program;
mod program_state;
mod shaders;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup_logger();
    program::run().await?;

    Ok(())
}
