// use tokio::io::AsyncReadExt;

// #[tokio::main]
// async fn main(){
//     let mut connection = tokio_quicker::QuicSocket::bind("127.0.0.1:0")
//         .await.unwrap()
//         .connect(Some("localhost"), "127.0.0.1:4433")
//         .await.unwrap();
    
//     let mut something = connection.incoming().await.unwrap();
//     let mut data = Vec::new();
//     something.read_buf(&mut data).await.unwrap();
//     println!("{:?}",data);
// }
#![allow(clippy::unused_io_amount)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_quicker::error::Result;
use tokio_quicker::{QuicSocket};

#[tokio::main]
async fn main() -> Result<()> {
    //simple_logger::SimpleLogger::new().init().unwrap();

    let mut connection = QuicSocket::bind("127.0.0.1:0")
        .await?
        .connect(Some("localhost"), "127.0.0.1:4433")
        .await?;

    let mut stream = connection.uni(1).await?;
    stream.write(b"./Cargo.toml").await?;

    // let mut buf: Vec<u8> = Vec::with_capacity(u16::MAX as usize * 4);
    // loop {
    //     if let Err(_) = stream.read_buf(&mut buf).await {
    //         break;
    //     }
    // }
    // println!("{}", String::from_utf8_lossy(&buf));
    stream.shutdown().await?;
    Ok(())
}