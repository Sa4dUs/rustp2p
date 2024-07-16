use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn send(address: String, file: String) {
    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);
    
    let mut stream = TcpStream::connect(&address).await.unwrap();
    println!("[rustp2p::commands::send.rs::send] Connected to the server at {}", address);

    stream.write_all("Hello World!".as_bytes()).await.unwrap();
}