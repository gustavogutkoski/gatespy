use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Scan ports from 1 to 1024
pub fn scan_host(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    for port in 1..=1024 {
        let addr = format!("{}:{}", host, port);

        // Resolves the host to a SocketAddr
        if let Ok(mut addrs) = addr.to_socket_addrs() {
            if let Some(socket_addr) = addrs.next() {
                if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).is_ok() {
                    println!("Port {} opened!", port);
                }
            }
        }
    }
    Ok(())
}
