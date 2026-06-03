use std::process::Command;

use crate::models::dns_model::WhoisInfo;

pub fn resolve_whois(domain: &str) -> WhoisInfo {
    let output = match Command::new("whois")
        .arg(domain)
        .output()
    {
        Ok(output) => output,

        Err(_) => {
            return WhoisInfo {
                registrar: "Unknown".to_string(),
                expire_date: "Unknown".to_string(),
                statuses: String::new(),
            };
        }
    };

    let response =
        String::from_utf8_lossy(&output.stdout);

    let mut registrar =
        String::new();

    let mut expire_date =
        String::new();

    let mut statuses =
        String::new();

    for line in response.lines() {
        let line = line.trim();
        // Registrar
        if line.starts_with("Registrar:")
        {
            registrar = line
                .replace("Registrar:", "")
                .trim()
                .to_string();
        }
        
        // Expiración
        if line.starts_with("Registry Expiry Date:")
            || line.starts_with("Expiration Date:")
            || line.starts_with("Expiry Date:")
        {
            expire_date = line
                .split(':')
                .skip(1)
                .collect::<Vec<_>>()
                .join(":")
                .trim()
                .to_string();
        }

        // Estados
        if line.starts_with("Domain Status:")
            || line.starts_with("Status:")
        {
            let status = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim();

            statuses.push_str(status);
            statuses.push('\n');
        }
    }
    if statuses.is_empty() {
        statuses = "Not Found".to_string();
    }
    WhoisInfo {
        registrar,
        expire_date,
        statuses,
    }

}