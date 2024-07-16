use rustp2p::cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    cli.run().await;
}
