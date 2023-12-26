use std::error::Error;

mod logging;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    logging::setup_logger();
    window::setup_window()?;

    Ok(())
}
