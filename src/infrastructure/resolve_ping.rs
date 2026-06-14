use std::{
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub fn resolve_ping(domain: &str) -> String {
    let mut child = match Command::new("ping")
        .args(["-c", "1", domain])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return "Error".to_string(),
    };

    let timeout = Duration::from_secs(2);
    let start = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return "Timeout".to_string();
                }

                let output = match child.wait_with_output() {
                    Ok(output) => output,
                    Err(_) => return "Error".to_string(),
                };

                let stdout =
                    String::from_utf8_lossy(&output.stdout);

                for line in stdout.lines() {
                    if let Some(pos) = line.find("time=") {
                        let value = &line[pos + 5..];

                        if let Some(end) = value.find(' ') {
                            return format!(
                                "{} ms",
                                &value[..end]
                            );
                        }
                    }
                }

                return "Responds".to_string();
            }

            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    return "Timeout".to_string();
                }

                thread::sleep(Duration::from_millis(50));
            }

            Err(_) => return "Error".to_string(),
        }
    }
}