use std::process::Command;

pub fn get_ptr(ip: &str) -> String {
    let ptr_output = Command::new("dig")
        .arg("+short")
        .arg("-x")
        .arg(ip)
        .output();

    match ptr_output {
        Ok(ptr_result) => {
            let ptrs: Vec<String> = String::from_utf8_lossy(&ptr_result.stdout)
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();

            if ptrs.is_empty() {
                return "PTR: no definido\n".to_string();
            }

            let mut result = String::from("PTR:\n");

            for p in ptrs {
                result.push_str(&format!(" - {}\n", p));
            }

            return result;
        }
        Err(e) => {
            return format!("PTR error: {}\n", e);
        }
    }
}

