use std::process::Command;

pub fn lookup_domain(domain: &str) -> String {

    // Hosts a consultar
    let hosts = vec![
        domain.to_string(),
        format!("www.{}", domain),
        format!("mail.{}", domain),
        format!("ftp.{}", domain),
        format!("webmail.{}", domain),
    ];

    // Resultado final
    let mut result_text = String::new();

    // Loop sobre hosts
    for host in hosts {

        // Ejecutar dig
        let output = Command::new("dig")
            .arg("+short")
            .arg(&host)
            .output();

        match output {

            Ok(result) => {

                let stdout =
                    String::from_utf8_lossy(&result.stdout);

                let lines: Vec<&str> =
                    stdout.lines().collect();

                // Si no hay resultado
                if lines.is_empty() {

                    result_text.push_str(
                        &format!(
                            "Host: {}\nNo encontrado\n\n",
                            host
                        )
                    );

                    continue;
                }

                // Mostrar resultados
                result_text.push_str(
                    &format!("Host: {}\n", host)
                );

                let mut first_ip: Option<String> = None;

                for line in lines {

                    let ip = line.trim();

                    // Validar IP
                    if ip.parse::<std::net::IpAddr>().is_ok() {

                        // Guardar SOLO la primera IP
                        if first_ip.is_none() {
                            first_ip = Some(ip.to_string());
                        }

                        // Mostrar TODAS las IPs
                        result_text.push_str(
                            &format!("Address: {}\n", ip)
                        );
                    }
                }

                // Después del loop
                // SOLO primera IP
                if let Some(ip) = first_ip {

                    // Consultar PTR
                    let ptr_output = Command::new("dig")
                        .arg("+short")
                        .arg("-x")
                        .arg(&ip)
                        .output();

                    match ptr_output {

                        Ok(ptr_result) => {

                            let ptr =
                                String::from_utf8_lossy(
                                    &ptr_result.stdout
                                );

                            let ptr = ptr.trim();

                            if !ptr.is_empty() {

                                result_text.push_str(
                                    &format!("PTR: {}\n", ptr)
                                );
                            }
                        }

                        Err(_) => {}
                    }

                    // Resolver usando ping SOLO una vez
                    let ping_output = Command::new("ping")
                        .arg("-c")
                        .arg("1")
                        .arg(&host)
                        .output();

                    match ping_output {

                        Ok(ping_result) => {

                            let ping_stdout =
                                String::from_utf8_lossy(
                                    &ping_result.stdout
                                );

                            let mut resolved_ip =
                                "No resuelve".to_string();

                            for line in ping_stdout.lines() {

                                if line.starts_with("PING") {

                                    if let Some(start) =
                                        line.find('(')
                                    {

                                        if let Some(end) =
                                            line.find(')')
                                        {

                                            resolved_ip =
                                                line[start + 1..end]
                                                .to_string();
                                        }
                                    }
                                }
                            }

                            result_text.push_str(
                                &format!(
                                    "Ping Resolve: {}\n",
                                    resolved_ip
                                )
                            );
                        }

                        Err(_) => {

                            result_text.push_str(
                                "Ping Resolve: No resuelve\n"
                            );
                        }
                    }
                }

                result_text.push('\n');
            }

            Err(error) => {

                result_text.push_str(
                    &format!(
                        "Host: {}\nError: {}\n\n",
                        host,
                        error
                    )
                );
            }
        }
    }

    result_text
}