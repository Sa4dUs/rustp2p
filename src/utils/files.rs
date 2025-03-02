use clap::Error;
use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

pub async fn read_to_buffer(file_path: &str, buffer: &mut [u8]) -> io::Result<usize> {
    let mut file = File::open(file_path).await?;
    let bytes_read = file.read(buffer).await?;
    Ok(bytes_read)
}

pub async fn write_from_buffer(filename: &str, buffer: &[u8]) -> Result<(), Error> {
    let mut f = File::create(filename).await?;
    f.write_all(buffer).await?;
    Ok(())
}
