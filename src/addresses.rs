use crate::ptr::get_ptr;
use std::process::Command;

pub fn get_addresses(host: &str) -> String {
    let mut result_text = String::new();

    let output = Command::new("dig")
        .arg("+short")
        .arg(host)
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let lines: Vec<&str> = stdout.lines().collect();

            let mut found_ip = false;
            let mut first_ip: Option<String> = None;

            for line in lines {
                let ip = line.trim();

                if ip.parse::<std::net::IpAddr>().is_ok() {
                    found_ip = true;

                    // guardar primera IP
                    if first_ip.is_none() {
                        first_ip = Some(ip.to_string());
                    }

                    result_text.push_str(&format!("Address: {}\n", ip));
                }
            }

            // 👇 usar la primera IP para PTR
            if let Some(ip) = first_ip {
                let ptr = get_ptr(&ip);
                result_text.push_str(&ptr);
            } else {
                result_text.push_str("PTR: no definido\n");
            }
            if !found_ip {
                result_text.push_str("No resuelve\n");
            }
        }

        Err(error) => {
            result_text.push_str(&format!("Error: {}\n", error));
        }
    }

    result_text
}