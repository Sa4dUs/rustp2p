use clap::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::utils::files::read_to_buffer;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn send(address: String, file: String) -> Result<(), Error> {
    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);
    
    let mut stream = TcpStream::connect(&address).await?;
    println!("[rustp2p::commands::send.rs::send] Connected to the server at {}", address);

    let mut file = File::open(file).await?;
    let mut buffer = [0u8; 1024];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n]).await?;
    }

    Ok(())
}
