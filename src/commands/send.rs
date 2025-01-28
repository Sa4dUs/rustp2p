use crate::utils::r#const::{BUFFER_SIZE, MAX_RETRIES};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send(address: String, filepath: String) -> Result<()> {
    println!(
        "[rustp2p::commands::send.rs::send] {} {}",
        address, filepath
    );

    let file_metadata = fs::metadata(&filepath).context("Failed to read file metadata")?;
    let file_size = file_metadata.len();
    let num_batches = (file_size + BUFFER_SIZE as u64 - 1) / BUFFER_SIZE as u64;
    let file_extension = Path::new(&filepath)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    let mut file = File::open(&filepath).await.context("Failed to open file")?;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    file.rewind().await.context("Failed to rewind file")?;

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

    let mut hasher = Sha256::new();

    while let Ok(n) = file.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);

        let mut retries = 0;

        loop {
            if retries >= MAX_RETRIES {
                return Err(anyhow::anyhow!(
                    "Failed to send batch after {} retries",
                    MAX_RETRIES
                ));
            }

            if let Err(e) = stream.write_all(&buffer[..n]).await {
                eprintln!("[rustp2p::commands::send.rs::send] Failed to write to stream, retrying...; err = {:?}", e);
                retries += 1;
                continue;
            }

            stream.flush().await.context("Failed to flush stream")?;

            let mut ack_buffer = [0u8; 4];
            match stream.read_exact(&mut ack_buffer).await {
                Ok(_) if &ack_buffer == b"ACK\n" => {
                    pb.inc(n as u64);
                    break;
                }
                Ok(_) => {
                    eprintln!("[rustp2p::commands::send.rs::send] Invalid acknowledgment received, retrying...");
                }
                Err(e) => {
                    eprintln!(
                        "[rustp2p::commands::send.rs::send] Failed to receive acknowledgment, retrying...; err = {:?}",
                        e
                    );
                }
            }

            retries += 1;
        }
    }

    let checksum = hasher.finalize();
    if let Err(e) = stream.write_all(&checksum).await {
        eprintln!(
            "[rustp2p::commands::send.rs::send] Failed to send checksum; err={}",
            e
        );
    }

    stream.flush().await.context("Failed to flush stream")?;
    pb.finish_with_message("File received");
    println!("[rustp2p::commands::send.rs::send] File transfer completed.");

    Ok(())
}
