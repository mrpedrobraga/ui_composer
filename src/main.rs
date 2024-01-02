use std::error::Error;

use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};

mod logging;
mod program;
mod renderer;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //logging::setup_logger();
    //program::run().await?;

    test_muts();

    Ok(())
}

fn test_muts() {
    let a = Mutable::new(10);
    let b = Mutable::new(20);
    let c = map_ref! {
        let a = a.signal(),
        let b = b.signal() =>
            *a + *b
    };

    a.set(10);
}
