use ui_composer::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = UIAppBuilder::new(())
        .build()
        .await?;
    
    app.run().await?;
    Ok(())
}