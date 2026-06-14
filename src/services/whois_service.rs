use std::process::Command;
use std::collections::HashSet;

use crate::models::dns_model::WhoisInfo;
use crate::utils::resolve_server_whois::resolve_server_whois;

const REGISTRAR_KEYS: [&str; 3] = [
    "Registrar:",
    "Registrar Name:",
    "Sponsoring Registrar:",
];

const EXPIRATION_KEYS: [&str; 6] = [
    "Registry Expiry Date:",
    "Expiration Date:",
    "Expiry date:",
    "Expiry Date:",
    "Expires On:",
    "Renewal Date:",
];

const STATUS_KEYS: [&str; 3] = [
    "status:",
    "Domain Status:",
    "Status:",
];


pub fn resolve_whois(domain: &str) -> WhoisInfo {

    let mut whois = WhoisInfo {
        registrar: "Unknown".to_string(),
        expire_date: "Unknown".to_string(),
        statuses: "Unknown".to_string(),
    };

    let server = resolve_server_whois(domain);

    if server == "Unknown" {
        return whois;
    }

    let output = match Command::new("whois")
        .args(["-h", &server, domain])
        .output()
    {
        Ok(output) => output,
        Err(_) => return whois,
    };

    let response =
        String::from_utf8_lossy(&output.stdout);

    whois.registrar = "Not Found".to_string();
    whois.expire_date = "Not Found".to_string();

    let mut statuses: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for line in response.lines() {

        let line = line.trim();

        // Registrar
        for key in REGISTRAR_KEYS {
            if line.starts_with(key) {

                whois.registrar = line
                    .trim_start_matches(key)
                    .trim()
                    .to_string();

                break;
            }
        }

        // Expiración
        for key in EXPIRATION_KEYS {
            if line.starts_with(key) {

                whois.expire_date = line
                    .trim_start_matches(key)
                    .trim()
                    .to_string();

                break;
            }
        }

        // Estados
        for key in STATUS_KEYS {
            if line.starts_with(key) {

                let status = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();

                if seen.insert(status.clone()) {
                    statuses.push(status);
                }

                break;
            }
        }
    }

    whois.statuses = if statuses.is_empty() {
        "Not Found".to_string()
    } else {
        statuses.join("\n")
    };

    whois
}
