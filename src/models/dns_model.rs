#[derive(Debug, Clone, Default)]
pub struct DnsQuery {
    pub domain: String,

    pub hosts: Vec<Host>,
}

#[derive(Debug, Clone, Default)]
pub struct Host {
    pub name: String,
    pub ip: String,
    pub ptr: String,
    pub ping: String,
    pub ssl: String,
}
