use std::process::Command;

pub fn resolve_server_whois(domain: &str) -> String {
    let output = match Command::new("whois")
        .args(["-h", "whois.iana.org", domain])
        .output()
    {
        Ok(output) => output,
        Err(_) => return "Unknown".to_string(),
    };

    if !output.status.success() {
        return "Unknown".to_string();
    }

    let response =
        String::from_utf8_lossy(&output.stdout);

    for line in response.lines() {
        let line = line.trim();

        if let Some(server) =
            line.strip_prefix("refer:")
        {
            return server.trim().to_string();
        }
    }

    "Unknown".to_string()
}