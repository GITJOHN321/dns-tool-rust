use std::process::Command;

pub fn resolve_spf(domain: &str) -> String {
    let output = Command::new("dig")
        .args(["+short", "TXT", domain])
        .output();

    let output = match output {
        Ok(output) => output,
        Err(_) => return "DNS Query Failed".to_string(),
    };

    let txt_records = String::from_utf8_lossy(&output.stdout);

    for line in txt_records.lines() {
        let record = line.trim_matches('"');

        if record.starts_with("v=spf1") {
            return record.to_string();
        }
    }

    "No SPF Record Found".to_string()
}