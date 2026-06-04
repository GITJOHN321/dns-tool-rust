use std::process::Command;

pub fn resolve_spf(domain: &str) -> String {
    let output = Command::new("nslookup")
        .args(["-type=TXT", domain])
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => return "DNS Query Failed".to_string(),
    };

    let response = String::from_utf8_lossy(&output.stdout);

    for line in response.lines() {
        let line = line.trim();

        if let Some(pos) = line.find("v=spf1") {
            return line[pos..]
                .trim_matches('"')
                .to_string();
        }
    }

    "No SPF Record Found".to_string()
}