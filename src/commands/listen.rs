use chrono::Utc;
use clap::Error;
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpListener};

pub async fn listen() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("[rustp2p::commands::listen.rs::listen] Server is listening on 0.0.0.0:8080");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!(
            "[rustp2p::commands::listen.rs::listen] (socket, addr) = ({:#?}, {})",
            socket, addr
        );

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            let mut file_buffer = Vec::new();

            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => {
                        println!("[rustp2p::commands::listen.rs::listen] Connection closed");
                        break;
                    }
                    Ok(n) => {
                        println!(
                            "[rustp2p::commands::listen.rs::listen] Received {} bytes",
                            n
                        );
                        file_buffer.extend_from_slice(&buffer[..n]);
                    }
                    Err(e) => {
                        println!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }

            if !file_buffer.is_empty() {
                write_to_single_file(addr, &file_buffer)
                    .await
                    .unwrap_or_else(|_| {
                        eprintln!("[rustp2p::commands::listen.rs::listen] Couldn't write to file")
                    });
            }
        });
    }
}

async fn write_to_single_file(addr: SocketAddr, buffer: &[u8]) -> Result<(), Error> {
    let filename = format!("{}-{}.dat", addr.ip(), Utc::now().timestamp());
    let mut f = File::create(filename).await?;
    f.write_all(buffer).await?;
    Ok(())
}
