use crate::utils::r#const::{BUFFER_SIZE, MAX_RETRIES, METADATA_PARTS};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::str;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn listen() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to bind to address")?;
    println!(
        "[rustp2p::commands::listen.rs::listen] Server is listening on {}",
        listener.local_addr().unwrap()
    );

    loop {
        let (mut socket, addr) = listener
            .accept()
            .await
            .context("Failed to accept connection")?;
        tokio::spawn(async move {
            let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
            let mut metadata_buffer = Vec::new();

            loop {
                match socket.read_buf(&mut metadata_buffer).await {
                    Ok(0) => {
                        println!("[rustp2p::commands::listen.rs::listen] Connection closed");
                        return;
                    }
                    Ok(_) => {
                        if let Some(pos) = metadata_buffer.iter().position(|&b| b == b'\n') {
                            println!(
                                "[rustp2p::commands::listen.rs::listen] Connection opened {}",
                                addr
                            );
                            let metadata_str = match str::from_utf8(&metadata_buffer[..pos]) {
                                Ok(s) => s,
                                Err(_) => {
                                    eprintln!("[rustp2p::commands::listen.rs::listen] Invalid metadata format");
                                    return;
                                }
                            };

                            metadata_buffer.clone().drain(..=pos);

                            let metadata_parts: Vec<&str> =
                                metadata_str.trim().split('|').collect();
                            if metadata_parts.len() != METADATA_PARTS {
                                eprintln!("[rustp2p::commands::listen.rs::listen] Invalid metadata format");
                                return;
                            }

                            let file_size: u64 = match metadata_parts[0].parse() {
                                Ok(size) => size,
                                Err(_) => {
                                    eprintln!(
                                        "[rustp2p::commands::listen.rs::listen] Invalid file size"
                                    );
                                    return;
                                }
                            };

                            let num_batches: u64 = match metadata_parts[1].parse() {
                                Ok(batches) => batches,
                                Err(_) => {
                                    eprintln!("[rustp2p::commands::listen.rs::listen] Invalid number of batches");
                                    return;
                                }
                            };

                            let file_extension = metadata_parts[3];
                            let file_name = format!("{}.{}", addr.ip(), file_extension);

                            let pb = ProgressBar::new(file_size);
                            pb.set_style(
                                ProgressStyle::default_bar()
                                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
                                    .expect("[rustp2p::commands::listen.rs::listen] Error building progress bar")
                                    .progress_chars("#>-"),
                            );

                            let mut hasher = Sha256::new();

                            let mut file = match File::create(file_name).await {
                                Ok(f) => f,
                                Err(e) => {
                                    eprintln!("[rustp2p::commands::listen.rs::listen] Failed to read checksum; err = {:?}", e);
                                    return;
                                }
                            };

                            for _ in 0..num_batches {
                                let mut retries = 0;
                                loop {
                                    if retries >= MAX_RETRIES {
                                        eprintln!("[rustp2p::commands::listen.rs::listen] Max retries reached for this batch. Aborting connection.");
                                        return;
                                    }

                                    let n = match socket.read(&mut buffer).await {
                                        Ok(n) => n,
                                        Err(e) => {
                                            eprintln!(
                                                "[rustp2p::commands::listen.rs::listen] Failed to read from socket; retrying...; err = {:?}",
                                                e
                                            );
                                            retries += 1;
                                            continue;
                                        }
                                    };

                                    if n == 0 {
                                        eprintln!("[rustp2p::commands::listen.rs::listen] Connection closed unexpectedly.");
                                        return;
                                    }

                                    hasher.update(&buffer[..n]);
                                    let _ = file.write_all(&buffer[..n]).await;
                                    pb.inc(n as u64);

                                    if let Err(e) = socket.write_all(b"ACK\n").await {
                                        eprintln!(
                                            "[rustp2p::commands::listen.rs::listen] Failed to send acknowledgment; err = {:?}",
                                            e
                                        );
                                        retries += 1;
                                    } else {
                                        break;
                                    }
                                }
                            }

                            let checksum = hasher.finalize();

                            let n = match socket.read(&mut buffer).await {
                                Ok(n) => n,
                                Err(e) => {
                                    eprintln!("[rustp2p::commands::listen.rs::listen] Failed to read checksum; err = {:?}", e);
                                    return;
                                }
                            };

                            if &buffer[..n] != checksum.as_slice() {
                                eprintln!("[rustp2p::commands::listen.rs::listen] Failed to read checksum.");
                                return;
                            }

                            println!(
                                "[rustp2p::commands::listen.rs::listen] File transfer succeded.",
                            );

                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
