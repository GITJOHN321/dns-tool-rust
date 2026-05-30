pub struct DomainInfo {
    pub domain: String,
    pub ptr: Option<String>,
    pub ping_resolve: Option<String>,
    pub hosts: Vec<HostInfo>,
}

pub struct HostInfo {
    pub hostname: String,
    pub addresses: Vec<String>,
}

pub fn build_domain(domain: &str) -> DomainInfo {

    DomainInfo {
        domain: domain.to_string(),
        ptr: None,
        ping_resolve: None,
        hosts: Vec::new(),
    }
}