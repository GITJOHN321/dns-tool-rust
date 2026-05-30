use std::process::Command;

/// Hace ping a un dominio y retorna el resultado como String
pub fn ping_domain(domain: &str) -> String {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(domain)
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                stdout.trim().to_string()
            } else {
                format!("Error al hacer ping:\n{}", stderr.trim())
            }
        }
        Err(e) => format!("No se pudo ejecutar ping: {}", e),
    }
}