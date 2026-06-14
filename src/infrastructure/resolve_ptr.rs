use std::process::Command;

pub fn resolve_ptr(hostname: &str) -> String {
    let output = Command::new("dig")
        .arg("+short")
        .arg("-x")
        .arg(&hostname)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if let Some(line) = stdout.lines().next() {
                line.trim().to_string()
            } else {
                "----".to_string()
            }
        }
        Err(e) => format!("{}", e),
    }
}