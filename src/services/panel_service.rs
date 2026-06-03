use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Verifica si un puerto TCP está abierto.
fn is_port_open(host: &str, port: u16) -> bool {
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

/// Detecta el panel basándose en puertos comunes.
pub fn detect_panel(domain: &str) -> String {
    // cPanel / WHM
    const CPANEL_PORTS: [u16; 4] = [2082, 2083, 2086, 2087];

    for port in CPANEL_PORTS {
        if is_port_open(domain, port) {
            return "cPanel".to_string();
        }
    }

    // Plesk
    if is_port_open(domain, 8443) {
        return "Plesk".to_string();
    }

    // DirectAdmin
    if is_port_open(domain, 2222) {
        return "DirectAdmin".to_string();
    }

    "Unknown".to_string()
}

