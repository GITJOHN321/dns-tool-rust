use std::process::Command;
use std::collections::HashSet;

use crate::models::dns_model::WhoisInfo;

const REGISTRAR_KEYS: [&str; 3] = [
    "Registrar:",
    "Registrar Name:",
    "Sponsoring Registrar:",
];

const EXPIRATION_KEYS: [&str; 5] = [
    "Registry Expiry Date:",
    "Expiration Date:",
    "Expiry Date:",
    "Expires On:",
    "Renewal Date:",
];

const STATUS_KEYS: [&str; 3] = [
    "Domain Status:",
    "Status:",
    "status:",
];


/*pub fn resolve_whois(domain: &str) -> WhoisInfo {
    WhoisInfo::default()
}*/
pub fn resolve_whois(domain: &str) -> WhoisInfo {

    let output = match Command::new("whois")
        .args(["-h","whois.iana.org", domain])
        .output()
    {
        Ok(output) => output,

        Err(_) => {
            return WhoisInfo {
                registrar: "Unknown".to_string(),
                expire_date: "Unknown".to_string(),
                statuses: "Unknown".to_string(),
            };
        }
    };

    let response =
        String::from_utf8_lossy(&output.stdout);

    let mut registrar =
        "Not Found".to_string();

    let mut expire_date =
        "Not Found".to_string();

    let mut statuses: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();


    for line in response.lines() {

        let line = line.trim();

        // Registrar
        for key in REGISTRAR_KEYS {

            if line.starts_with(key) {

                registrar = line
                    .trim_start_matches(key)
                    .trim()
                    .to_string();

                break;
            }
        }

        // Expiración
        for key in EXPIRATION_KEYS {

            if line.starts_with(key) {

                expire_date = line
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

    let statuses = if statuses.is_empty() {
        "Not Found".to_string()
    } else {
        statuses.join("\n")
    };

    WhoisInfo {
        registrar,
        expire_date,
        statuses,
    }
}
