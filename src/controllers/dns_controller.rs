// controllers/dns_controller.rs
use crate::models::dns_model::DnsQuery;
use crate::services::dns_service::query_domain;
use crate::services::ns_service::resolve_ns;
use crate::services::mx_service::resolve_mx;
use crate::services::panel_service::detect_panel;
use crate::services::spf_service::resolve_spf;

pub fn execute_query(domain: &str) -> DnsQuery {
    // Aquí puedes validar el dominio,
    // registrar logs, combinar servicios, etc.

    DnsQuery{
        domain: domain.to_string(),
        hosts: query_domain(&domain),
        ns: resolve_ns(&domain),
        mx: resolve_mx(&domain),
        panel: detect_panel(&domain),
        spf: resolve_spf(&domain),
    }
}
  
/*
use std::thread;

pub fn execute_query(domain: &str) -> DnsQuery {
    let domain = domain.to_string();

    let hosts_domain = domain.clone();
    let ns_domain = domain.clone();
    let mx_domain = domain.clone();
    let panel_domain = domain.clone();

    let hosts_handle = thread::spawn(move || {
        query_domain(&hosts_domain)
    });

    let ns_handle = thread::spawn(move || {
        resolve_ns(&ns_domain)
    });

    let mx_handle = thread::spawn(move || {
        resolve_mx(&mx_domain)
    });

    let panel_handle = thread::spawn(move || {
        detect_panel(&panel_domain)
    });

    DnsQuery {
        domain,
        hosts: hosts_handle.join().unwrap(),
        ns: ns_handle.join().unwrap(),
        mx: mx_handle.join().unwrap(),
        panel: panel_handle.join().unwrap(),
    }
}
*/