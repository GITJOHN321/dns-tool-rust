use crate::models::dns_model::DnsQuery;

pub fn format_dns_query(q: &DnsQuery) -> String {
    let mut out = String::new();

    out.push_str(&format!("DOMAIN: {}\n\n", q.domain));

    out.push_str("=== HOSTS ===\n");
    for h in &q.hosts {
        out.push_str(&format!(
            "- {}:\n{} -> {} | {}\n",
            h.name, h.ip, h.ptr, h.ping
        ));
        out.push_str("=== SSL ===\n");
        out.push_str(&format!(
            "  SSL: {} | {} | {}\n",
            h.ssl.organization, h.ssl.date, h.ssl.active
        ));
    }

    out.push_str("\n=== NS ===\n");
    out.push_str(&q.ns);

    out.push_str("\n\n=== MX ===\n");
    out.push_str(&q.mx);

    out.push_str("\n\n=== PANEL ===\n");
    out.push_str(&q.panel);

    out.push_str("\n\n=== SPF ===\n");
    out.push_str(&q.spf);

    out.push_str("\n\n=== DKIM ===\n");
    out.push_str(&q.dkim);

    out.push_str("\n\n=== DMARC ===\n");
    out.push_str(&q.dmarc);

    out.push_str("\n\n=== WHOIS ===\n");
    out.push_str(&format!(
        "Registrar: {}\nExpire: {}\nStatuses:\n{}",
        q.whois.registrar, q.whois.expire_date, q.whois.statuses
    ));

    out
}