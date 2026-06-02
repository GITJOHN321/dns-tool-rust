// controllers/dns_controller.rs
use crate::models::dns_model::DnsQuery;
use crate::services::dns_service;

pub fn execute_query(domain: &str) -> DnsQuery {
    // Aquí puedes validar el dominio,
    // registrar logs, combinar servicios, etc.

    DnsQuery{
        domain: domain.to_string(),
        hosts: dns_service::query_domain(domain)
    }
}
  