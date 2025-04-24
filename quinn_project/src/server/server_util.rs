use quinn::{Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
// use rcgen::
use std::{error::Error, net::SocketAddr, sync::Arc};

#[allow(unused)]
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let (mut server_config, server_cert) = configure_server()?;
    // server_config.transport_config()
    let mut transport_config = quinn::TransportConfig::default();
    transport_config.max_idle_timeout(Some(tokio::time::Duration::from_secs(30).try_into().unwrap()));
    transport_config.keep_alive_interval(Some(tokio::time::Duration::from_secs(15)));
    // server_config.
    server_config.transport_config(transport_config.into());
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    // endpoint.set_server_config(server_config);
    Ok((endpoint, server_cert))
}

fn configure_server()
-> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
