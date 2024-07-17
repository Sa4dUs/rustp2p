use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::utils::r#const::BUFFER_SIZE;

pub async fn send(address: String, file: String) -> Result<()> {
    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);

    let file_metadata = fs::metadata(&file).context("Failed to read file metadata")?;
    let file_size = file_metadata.len();
    let num_batches = (file_size + BUFFER_SIZE as u64 - 1) / BUFFER_SIZE as u64;
    let file_extension = Path::new(&file)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    let mut file = File::open(&file).await.context("Failed to open file")?;
    let mut stream = TcpStream::connect(&address)
        .await
        .context("Failed to connect to server")?;
    println!(
        "[rustp2p::commands::send.rs::send] Connected to the server at {}",
        address
    );

    let metadata = format!(
        "{}|{}|{}|{}\n",
        file_size, num_batches, BUFFER_SIZE, file_extension
    );
    stream
        .write_all(metadata.as_bytes())
        .await
        .context("Failed to write metadata to stream")?;
    stream.flush().await.context("Failed to flush stream")?;

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
            .expect("Error building progress bar")
            .progress_chars("#>-"),
    );

    let mut buffer = vec![0u8; BUFFER_SIZE];

    while let Ok(n) = file.read(&mut buffer).await {
        if n == 0 {
            break;
        }
        stream
            .write_all(&buffer[..n])
            .await
            .context("Failed to write to stream")?;
        pb.inc(n as u64);
    }

    stream.flush().await.context("Failed to flush stream")?;
    pb.finish_with_message("File received");
    println!("[rustp2p::commands::send.rs::send] File transfer completed.");

    Ok(())
}
