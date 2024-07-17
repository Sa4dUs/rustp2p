use clap::Error;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn send(address: String, file: String) -> Result<(), Error> {
    println!("[rustp2p::commands::send.rs::send] {} {}", address, file);

    let file_metadata = fs::metadata(&file)?;
    let file_size = file_metadata.len();
    let batch_size = 1024;
    let num_batches = (file_size + batch_size as u64 - 1) / batch_size as u64;
    let file_extension = Path::new(&file)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let mut file = File::open(&file).await?;

    let mut stream = TcpStream::connect(&address).await?;
    println!(
        "[rustp2p::commands::send.rs::send] Connected to the server at {}",
        address
    );

    let metadata = format!(
        "{}|{}|{}|{}\n",
        file_size, num_batches, batch_size, file_extension
    );
    stream.write_all(metadata.as_bytes()).await?;

    let mut buffer = [0u8; 1024];
    let pb = Arc::new(Mutex::new(ProgressBar::new(file_size)));
    pb.lock().unwrap().set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
            .expect("[rustp2p::commands::listen.rs::listen] Error buliding progressbar")
            .progress_chars("#>-"),
    );

    let pb_clone = pb.clone();

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n]).await?;
        pb_clone.lock().unwrap().inc(n as u64);
    }

    stream.flush().await?;
    pb_clone
        .lock()
        .unwrap()
        .finish_with_message("File received");
    println!("[rustp2p::commands::send.rs::send] File transfer completed.");

    Ok(())
}
