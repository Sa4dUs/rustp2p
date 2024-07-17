use std::time::Instant;
use anyhow::Result;
use rustp2p::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();

    let start = Instant::now();
    cli.run().await?;
    let duration = start.elapsed();

    println!("Time elapsed {:?}", duration);
    Ok(())
}
