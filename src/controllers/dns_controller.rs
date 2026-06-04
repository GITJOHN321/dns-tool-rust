// controllers/dns_controller.rs
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use crate::models::dns_model::DnsQuery;
use crate::models::dns_model::WhoisInfo;
use crate::services::{dns_service,ns_service, mx_service,panel_service,spf_service,dkim_service,dmarc_service,whois_service};

pub fn execute_query(domain: &str) -> DnsQuery {
    // Aquí puedes validar el dominio,
    // registrar logs, combinar servicios, etc.
    let domain = domain.to_string();
    let hosts_domain = domain.clone();
    let ns_domain = domain.clone();
    let mx_domain = domain.clone();
    let panel_domain = domain.clone();
    let spf_domain = domain.clone();
    let dkim_domain = domain.clone();
    let dmarc_domain = domain.clone();
    let whois_domain = domain.clone();

    let hosts_handle = thread::spawn(move || {
        dns_service::query_domain(&hosts_domain)
    });

    let ns_handle = thread::spawn(move || {
        ns_service::resolve_ns(&ns_domain)
    });

    let mx_handle = thread::spawn(move || {
        mx_service::resolve_mx(&mx_domain)
    });

    let panel_handle = thread::spawn(move || {
        panel_service::detect_panel(&panel_domain)
    });
    let spf_handle = thread::spawn(move || {
        spf_service::resolve_spf(&spf_domain)
    });
    let dkim_handle = thread::spawn(move || {
        dkim_service::resolve_dkim(&dkim_domain)
    });
    let dmarc_handle = thread::spawn(move || {
        dmarc_service::resolve_dmarc(&dmarc_domain)
    });
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = whois_service::resolve_whois(&whois_domain);
        let _ = tx.send(result);
    });

    let whois_result = match rx.recv_timeout(Duration::from_secs(3)) {
        Ok(res) => res,
        Err(_) => WhoisInfo {
            registrar: "Timeout".to_string(),
            expire_date: "Timeout".to_string(),
            statuses: "Timeout".to_string(),
        },
    };

    DnsQuery{
        domain: domain.to_string(),
        hosts: hosts_handle.join().unwrap(),
        ns: ns_handle.join().unwrap(),
        mx: mx_handle.join().unwrap(),
        panel: panel_handle.join().unwrap(),
        spf: spf_handle.join().unwrap(),
        dkim: dkim_handle.join().unwrap(),
        dmarc: dmarc_handle.join().unwrap(),
        whois: whois_result,
    }
}
  