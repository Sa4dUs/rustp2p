use anyhow::{Context, Result};
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use std::str;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::utils::r#const::BUFFER_SIZE;
use crate::utils::files::write_from_buffer;

pub async fn listen() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .context("Failed to bind to address")?;
    println!("[rustp2p::commands::listen.rs::listen] Server is listening on {}", listener.local_addr().unwrap());

    loop {
        let (mut socket, addr) = listener
            .accept()
            .await
            .context("Failed to accept connection")?;
        tokio::spawn(async move {
            let mut buffer = vec![0; BUFFER_SIZE];
            let mut metadata_buffer = Vec::new();

            loop {
                match socket.read_buf(&mut metadata_buffer).await {
                    Ok(0) => {
                        println!("[rustp2p::commands::listen.rs::listen] Connection closed");
                        return;
                    }
                    Ok(_) => {
                        if let Some(pos) = metadata_buffer.iter().position(|&b| b == b'\n') {
                            println!("[rustp2p::commands::listen.rs::listen] Connection opened");
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
                            if metadata_parts.len() != 4 {
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

                            let filename = format!(
                                "{}-{}.{}",
                                addr.ip(),
                                Utc::now().timestamp(),
                                file_extension
                            );

                            let pb = ProgressBar::new(file_size);
                            pb.set_style(
                                ProgressStyle::default_bar()
                                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
                                    .expect("[rustp2p::commands::listen.rs::listen] Error building progress bar")
                                    .progress_chars("#>-"),
                            );

                            let mut file_buffer = Vec::with_capacity(file_size as usize);

                            for _ in 0..num_batches {
                                let n = match socket.read(&mut buffer).await {
                                    Ok(n) => n,
                                    Err(e) => {
                                        eprintln!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                                        return;
                                    }
                                };

                                if n == 0 {
                                    break;
                                }

                                file_buffer.extend_from_slice(&buffer[..n]);
                                pb.inc(n as u64);
                            }

                            if let Err(e) = write_from_buffer(&filename, &file_buffer).await {
                                eprintln!("[rustp2p::commands::listen.rs::listen] Failed to write to file; err = {:?}", e);
                            }

                            pb.finish_with_message("File received");
                            println!(
                                "[rustp2p::commands::listen.rs::listen] File transfer completed"
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
