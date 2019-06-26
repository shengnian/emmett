// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-graphite.html

use std::path::Path;

/// Receive graphite metrics. This plugin understands the text-based graphite carbon protocol.
pub struct Graphite {
    pub host: Option<String>,
    pub mode: Option<String>,
    pub port: u64,
    pub proxy_protocol: Option<bool>,
    pub ssl_cert: Option<&'static Path>,
    pub ssl_enable: Option<bool>,
    pub ssl_extra_chain_certs: Option<Vec<String>>,
    pub ssl_key: Option<&'static Path>,
    pub ssl_key_passphrase: Option<String>,
    pub ssl_verify: Option<bool>,
}
