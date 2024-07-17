use chrono::Utc;
use clap::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::str;
use std::sync::{Arc, Mutex};
use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::utils::files::write_from_buffer;

pub async fn listen() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("[rustp2p::commands::listen.rs::listen] Server is listening on 0.0.0.0:8080");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            let mut metadata_buffer = vec![];

            loop {
                match socket.read_buf(&mut metadata_buffer).await {
                    Ok(0) => {
                        println!("[rustp2p::commands::listen.rs::listen] Connection closed");
                        return;
                    }
                    Ok(_) => {
                        if let Some(pos) = metadata_buffer.iter().position(|&b| b == b'\n') {
                            println!("[rustp2p::commands::listen.rs::listen] Connection opened");
                            let metadata_buffer_clone = metadata_buffer.clone();
                            let metadata_str = match str::from_utf8(&metadata_buffer_clone[..pos]) {
                                Ok(s) => s,
                                Err(_) => {
                                    println!("[rustp2p::commands::listen.rs::listen] Invalid metadata format");
                                    return;
                                }
                            };

                            metadata_buffer.drain(..=pos);

                            let metadata_parts: Vec<&str> =
                                metadata_str.trim().split('|').collect();
                            if metadata_parts.len() != 4 {
                                println!("[rustp2p::commands::listen.rs::listen] Invalid metadata format");
                                return;
                            }
                            let file_size: u64 = metadata_parts[0].parse().unwrap_or(0);
                            let num_batches: u64 = metadata_parts[1].parse().unwrap_or(0);
                            let _batch_size: usize = metadata_parts[2].parse().unwrap_or(0);
                            let file_extension = metadata_parts[3];

                            let filename = format!(
                                "{}-{}.{}",
                                addr.ip(),
                                Utc::now().timestamp(),
                                file_extension
                            );

                            let pb = Arc::new(Mutex::new(ProgressBar::new(file_size)));
                            pb.lock().unwrap().set_style(ProgressStyle::default_bar()
                                .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})").expect("[rustp2p::commands::listen.rs::listen] Error buliding progressbar")
                                .progress_chars("#>-"));

                            let pb_clone = pb.clone();

                            tokio::spawn(async move {
                                let mut file_buffer = Vec::with_capacity(file_size as usize);
                                for _ in 0..num_batches {
                                    let n = socket.read(&mut buffer).await.unwrap_or(0);
                                    if n == 0 {
                                        break;
                                    }
                                    file_buffer.extend_from_slice(&buffer[..n]);
                                    pb_clone.lock().unwrap().inc(n as u64);
                                }

                                write_from_buffer(&filename, &file_buffer).await.unwrap_or_else(|_| eprintln!("[rustp2p::commands::listen.rs::listen] Couldn't write to file"));
                                pb_clone
                                    .lock()
                                    .unwrap()
                                    .finish_with_message("File received");

                                println!(
                                    "[rustp2p::commands::listen.rs::listen] Connection closed"
                                );
                            });

                            return;
                        }
                    }
                    Err(e) => {
                        println!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
