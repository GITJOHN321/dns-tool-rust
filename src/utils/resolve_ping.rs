use std::process::Command;

pub fn resolve_ping(domain: &str) -> String {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg("1")
        .arg(domain)
        .output();

    match output {
        Ok(output) => {

            // Si ping devolvió error o timeout
            if !output.status.success() {
                return "No responde".to_string();
            }

            let stdout =
                String::from_utf8_lossy(&output.stdout);

            // Buscar la línea que contiene time=
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

            // Respondió pero no se pudo extraer el tiempo
            "Responde".to_string()
        }

        Err(error) => {
            format!("Error: {}", error)
        }
    }
}