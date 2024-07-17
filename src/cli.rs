use crate::commands::{listen::listen, send::send};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    Send {
        #[clap(long, short = 'a')]
        address: String,
        #[clap(long, short = 'f')]
        file: String,
    },
    Listen,
}

#[derive(Parser)]
#[clap(name = "rustp2p", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub async fn run(self) -> Result<()> {
        match self.command {
            Command::Send { address, file } => {
                send(address, file).await?;
            }
            Command::Listen => {
                listen().await?;
            }
        }

        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}
