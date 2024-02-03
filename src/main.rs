mod modules;
use modules::{Amazon, Instagram};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut instagram = Instagram::new();
    let mut amazon = Amazon::new();
    // instagram.start("+33781403245").await?;
    amazon.start("+33781403245").await?;
    Ok(())
}
