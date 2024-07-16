use clap::{Parser, Subcommand};

use crate::commands::{listen::listen, send::send};

#[derive(Subcommand)]
pub enum Command {
    Address,
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

    pub async fn run(self) {
        match self.command {
            Command::Address => {
                println!("[rustp2p::Comand::Address]");
            }
            Command::Send { address, file } => {
                send(address, file).await;
            }
            Command::Listen => {
                listen().await;
            }
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}
