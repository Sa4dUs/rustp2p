use clap::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::utils::files::read_to_buffer;

pub async fn send(address: String, file: String) -> Result<(), Error> {
    let mut buffer:[u8; 1024] = [0; 1024];
    let n = read_to_buffer(&file, &mut buffer).await?;

    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);
    
    let mut stream = TcpStream::connect(&address).await?;
    println!("[rustp2p::commands::send.rs::send] Connected to the server at {}", address);

    stream.write_all(&buffer[..n]).await?;

    Ok(())
}