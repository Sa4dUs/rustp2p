use clap::Error;
use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::utils::files::write_from_buffer;


pub async fn listen() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("[rustp2p::commands::listen.rs::listen] Server is listening on 0.0.0.0:8080");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("[rustp2p::commands::listen.rs::listen] (socket, addr) = ({:#?}, {})", socket, addr);

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            match socket.read(&mut buffer).await {
                Ok(0) => {
                    println!("[rustp2p::commands::listen.rs::listen] Connection closed");
                }
                Ok(n) => {
                    println!("[rustp2p::commands::listen.rs::listen] Received {} bytes: {:?}", n, &buffer[..n]);

                    write_from_buffer(addr, &buffer[..n]).await.unwrap_or_else(|_| eprintln!("[rustp2p::commands::listen.rs::listen] Couldn't write to file"));
                }
                Err(e) => {
                    println!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}
