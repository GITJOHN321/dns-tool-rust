use std::{
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::models::dns_model::Ssl;

pub fn resolve_ssl(domain: &str) -> Ssl {

    let command = format!(
        "echo | openssl s_client \
        -connect {0}:443 \
        -servername {0} 2>/dev/null \
        | openssl x509 -noout -issuer -dates",
        domain
    );

    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,

        Err(_) => {
            return Ssl {
                date: String::new(),
                organization: String::new(),
                active: "Error".to_string(),
            };
        }
    };

    let timeout = Duration::from_secs(1);
    let start = Instant::now();

    loop {

        match child.try_wait() {

            // El proceso terminó
            Ok(Some(status)) => {

                if !status.success() {

                    return Ssl {
                        date: String::new(),
                        organization: String::new(),
                        active: "Inactive".to_string(),
                    };
                }

                let output = match child.wait_with_output() {
                    Ok(output) => output,

                    Err(_) => {
                        return Ssl {
                            date: String::new(),
                            organization: String::new(),
                            active: "Error".to_string(),
                        };
                    }
                };

                let stdout =
                    String::from_utf8_lossy(&output.stdout);

                let mut expire_date =
                    String::new();

                let mut organization =
                    String::new();

                for line in stdout.lines() {

                    if line.starts_with("issuer=") {

                        if let Some(pos) = line.find("O=") {

                            let issuer =
                                &line[pos + 2..];

                            organization =
                                issuer
                                    .split(',')
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string();
                        }
                    }

                    if line.starts_with("notAfter=") {

                        expire_date =
                            line
                                .replace("notAfter=", "")
                                .trim()
                                .to_string();
                    }
                }

                return Ssl {
                    date: expire_date,
                    organization,
                    active: "OK".to_string(),
                };
            }

            // Sigue ejecutándose
            Ok(None) => {

                if start.elapsed() >= timeout {

                    let _ = child.kill();

                    return Ssl {
                        date: String::new(),
                        organization: String::new(),
                        active: "Timeout".to_string(),
                    };
                }

                thread::sleep(
                    Duration::from_millis(100)
                );
            }

            // Error consultando estado
            Err(_) => {

                return Ssl {
                    date: String::new(),
                    organization: String::new(),
                    active: "Error".to_string(),
                };
            }
        }
    }
}