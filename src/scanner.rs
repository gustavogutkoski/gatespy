use std::net::{IpAddr, TcpStream, ToSocketAddrs, SocketAddr};
use std::time::Duration;

/// Scan ports from 1 to 1024
pub fn scan_host(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ips: Vec<IpAddr> = (host, 0)
        .to_socket_addrs()?
        .map(|addr| addr.ip())
        .collect();

    let socket_addrs: Vec<SocketAddr> = ips
        .iter()
        .flat_map(|ip| (1..=1024).map(move |port| SocketAddr::new(*ip, port)))
        .collect();

    for addr in socket_addrs {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
            println!("Port {} open on {}", addr.port(), addr.ip());
        }
    }

    Ok(())
}