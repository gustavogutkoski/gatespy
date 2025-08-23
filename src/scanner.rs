use std::net::{IpAddr, TcpStream, ToSocketAddrs, SocketAddr};
use std::time::Duration;

/// Scan ports from 1 to 1024
pub fn scan_host(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Resolve o host uma vez só
    let ips: Vec<IpAddr> = (host, 0) // porta dummy
        .to_socket_addrs()? // resolve host
        .map(|addr| addr.ip())
        .collect();

    // Pré-monta todos os SocketAddr (ip + porta)
    let socket_addrs: Vec<SocketAddr> = ips
        .iter()
        .flat_map(|ip| (1..=1024).map(move |port| SocketAddr::new(*ip, port)))
        .collect();

    // Agora só testa cada socket
    for addr in socket_addrs {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
            println!("Port {} open on {}", addr.port(), addr.ip());
        }
    }

    Ok(())
}