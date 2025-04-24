mod server_util;
use std::{error::Error, net::{IpAddr, Ipv4Addr, SocketAddr}, vec};
use rustls::crypto::ring::default_provider;
use server_util::make_server_endpoint;
use tokio::io::AsyncReadExt;
#[tokio::main]
async fn main()-> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    default_provider().install_default().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
    let (mut endpoint, _cert) = make_server_endpoint(addr).unwrap();
    // endpoint.wait_idle().await;
    let incoming = endpoint.accept().await.unwrap();
    let conn = incoming.await.unwrap();
    let mut stream = conn.accept_bi().await.unwrap();
    println!("accepted from client {}", conn.remote_address());
    loop{
        let mut buf = Vec::with_capacity(100);
        let res= stream.1.read_buf(&mut buf).await;
        match res{
            Ok(n)=>{
                println!("{}: {:?} with {}",stream.1.id(), buf, n);
                if n<=0{
                    break;
                }
            },
            Err(e)=>{
                break;
            }
        }   
    }
    stream.0.finish();
    conn.close(0u8.into(), b"done");
    Ok(())
    // endpoint
    
}