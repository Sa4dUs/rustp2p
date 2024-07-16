use tokio::{io::AsyncReadExt, net::TcpListener};

pub async fn listen() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("[rustp2p::commands::listen.rs::listen] Server is listening on 127.0.0.1:8080");

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

                    let byte_array = &buffer[..n];
                }
                Err(e) => {
                    println!("[rustp2p::commands::listen.rs::listen] Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}
