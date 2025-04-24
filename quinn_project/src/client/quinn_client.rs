mod client_util;
use std::{error::Error, net::IpAddr};

use client_util::{run_unsafe_client, setup_unsafe};
use tokio::io::AsyncWriteExt;
#[tokio::main]
async fn main()-> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use rustls::crypto::ring::default_provider;
    default_provider().install_default().unwrap();
    // make_client_endpoint(bind_addr, server_certs)
    let addr = std::net::SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 8080);
    // run_unsafe_client(addr);
    // setup_unsafe(addr);
    let (mut connection, mut endpoint) = setup_unsafe(addr).await?;

    //start the stream from client
    let mut stream = connection.open_bi().await?;
    // stream.1.received_reset()
    // stream.0.stopped()
    
    for i in 0..100{
        stream.0.write(&[i]).await;
        stream.0.flush().await;
    }
    stream.0.finish();
    stream.0.shutdown().await;
    // stream.1.stop();
    // stream.0.finish();
    // connection.close(0u8.into(), b"done");
    endpoint.wait_idle().await;
    Ok(())
}