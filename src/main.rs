use std::error::Error;

mod logging;
mod program;
mod shaders;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup_logger();
    program::run().await?;

    Ok(())
}
