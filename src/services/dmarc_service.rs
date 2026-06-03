use std::process::Command;

pub fn resolve_dmarc(domain: &str) -> String {
    let host = format!("_dmarc.{domain}");

    let output = match Command::new("dig")
        .args(["+short", "TXT", &host])
        .output()
    {
        Ok(output) => output,
        Err(_) => return "DNS Query Failed".to_string(),
    };

    let response = String::from_utf8_lossy(&output.stdout);

    for line in response.lines() {
        let record = line.replace('"', "");

        if record.trim_start().starts_with("v=DMARC1") {
            return record.trim().to_string();
        }
    }

    "Not Found".to_string()
}