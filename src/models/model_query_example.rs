#[derive(Debug, Clone, Default)]
pub struct DnsQuery {
    pub domain: String,

    pub hosts: Vec<Host>,

    pub spf: String,
    pub dmarc: String,
    pub dkim: String,

    pub ns: String,
    pub mx: String,

    pub panel: Panel,
    pub whois: Whois,

    pub details: String,
}

#[derive(Debug, Clone, Default)]
pub struct Host {
    pub name: String,
    pub ip: String,
    pub ptr: String,
    pub ping: String,
    pub ssl: Ssl,
}

#[derive(Debug, Clone, Default)]
pub struct Ssl {
    pub date: String,
    pub organization: String,
    pub active: String,
}

#[derive(Debug, Clone, Default)]
pub struct Panel {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Default)]
pub struct Whois {
    pub date_register: String,
    pub date_expire: String,
    pub date_update: String,
    pub status: String,
}