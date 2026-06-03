use std::process::Command;

pub fn resolve_dkim(domain: &str) -> String {
    let selectors = [
        "default",
        "selector1",
        "selector2",
        "google",
    ];

    for selector in selectors {
        let host = format!("{selector}._domainkey.{domain}");

        let output = match Command::new("dig")
            .args(["+short", "TXT", &host])
            .output()
        {
            Ok(output) => output,
            Err(_) => continue,
        };

        let response = String::from_utf8_lossy(&output.stdout);

        if response.contains("v=DKIM1") {
            return response.trim().to_string();
        }
    }

    "Not Found".to_string()
}