mod modules;
use modules::Instagram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut aha = Instagram::new();
    aha.start().await?;
    Ok(())
}
