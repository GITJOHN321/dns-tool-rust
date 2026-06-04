use std::process::Command;

pub fn resolve_dmarc(domain: &str) -> String {
    let host = format!("_dmarc.{domain}");
    let output = Command::new("nslookup")
        .args(["-type=TXT", &host])
        .output();

    let output = match output {
        Ok(output) => output,
        Err(_) => return "DNS Query Failed".to_string(),
    };

    let response = String::from_utf8_lossy(&output.stdout);

    for line in response.lines() {
        if line.contains("v=DMARC1") {
            return line
                .trim_matches('"')
                .to_string();
        }
    }

    "No DMARC Record Found".to_string()
}