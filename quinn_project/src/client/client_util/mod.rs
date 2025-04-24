use quinn::{ClientConfig, Connection, Endpoint, crypto::rustls::QuicClientConfig};
use rustls::pki_types::CertificateDer;
// use rcgen::
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
pub mod cert;
use cert::SkipServerVerification;
#[allow(unused)]
pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}
fn configure_client(
    server_certs: &[&[u8]],
) -> Result<ClientConfig, Box<dyn Error + Send + Sync + 'static>> {
    // default_provider().install_default().unwrap();
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(CertificateDer::from(*cert))?;
    }

    Ok(ClientConfig::with_root_certificates(Arc::new(certs))?)
}

pub async fn setup_unsafe(
    server_addr: SocketAddr,
) -> Result<(Connection, Endpoint), Box<dyn Error + Send + Sync + 'static>> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    endpoint.set_default_client_config(ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?)));

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    Ok((connection, endpoint))
}

pub async fn run_unsafe_client(
    server_addr: SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    endpoint.set_default_client_config(ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?)));

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());
    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);
    // Make sure the server has a chance to clean up

    endpoint.wait_idle().await;

    Ok(())
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
