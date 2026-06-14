use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn resolve_port(host: &str, port: u16) -> bool {
    let addr = format!("{host}:{port}");

    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            return TcpStream::connect_timeout(
                &addr,
                Duration::from_millis(800),
            )
            .is_ok();
        }
    }

    false
}