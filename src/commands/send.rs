use clap::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use core::time;
use std::{fs, thread};
use std::path::Path;

pub async fn send(address: String, file: String) -> Result<(), Error> {
    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);

    let file_metadata = fs::metadata(&file)?;
    let file_size = file_metadata.len();
    let batch_size = 1024;
    let num_batches = (file_size + batch_size as u64 - 1) / batch_size as u64;
    let file_extension = Path::new(&file).extension().unwrap_or_default().to_str().unwrap_or_default();
    let mut file = File::open(&file).await?;

    let mut stream = TcpStream::connect(&address).await?;
    println!("[rustp2p::commands::send.rs::send] Connected to the server at {}", address);

    let metadata = format!("{}|{}|{}|{}\n", file_size, num_batches, batch_size, file_extension);
    stream.write_all(metadata.as_bytes()).await?;

    let mut buffer = [0u8; 1024];
    let mut count = 1;

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n]).await?;
        println!("[rustp2p::commands::send.rs::send] ({}/{})", count, num_batches);
        thread::sleep(time::Duration::from_millis(100));
        count += 1;
    }

    stream.flush().await?;
    println!("[rustp2p::commands::send.rs::send] File transfer completed.");

    Ok(())
}
