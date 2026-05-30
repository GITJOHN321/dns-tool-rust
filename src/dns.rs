use crate::addresses::get_addresses;
use crate::ping::ping_domain;
//use std::process::Command;
pub fn lookup_domain(domain: &str) -> String {

    // Hosts a consultar
    let hosts = [
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

        result_text.push_str(
            &format!("Host: {}\n", host)
        );

        result_text.push_str(
           &get_addresses(&host)
        );
        //result_text.push_str(
          // &ping_domain(&host)
        //);

        result_text.push('\n');


    }

    result_text
}


