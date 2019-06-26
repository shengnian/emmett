// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-beats.html

#[derive(Debug)]
/// This input plugin enables Logstash to receive events from the Elastic Beats framework.
pub struct Beats {
    pub add_hostname: Option<bool>,
    pub cipher_suites: Option<Vec<&'static str>>,
    pub client_inactivity_timeout: Option<u64>,
    pub host: Option<String>,
    pub include_codec_tag: Option<String>,
    pub port: Option<String>,
    pub ssl: Option<bool>,
    pub ssl_certificate: Option<String>,
    pub ssl_certificate_authorities: Option<u64>,
    pub ssl_handshake_timeout: Option<u64>,
    pub ssl_key: Option<String>,
    pub ssl_key_passphrase: Option<String>,
    pub ssl_verify_mode: Option<u64>,
    pub ssl_peer_metadata: Option<u64>,
    pub tls_max_version: Option<u64>,
    pub tls_min_version: Option<u64>,
}

impl Default for Beats {
    fn default() -> Self {
        Self {
            add_hostname: None,
            cipher_suites: None,
            client_inactivity_timeout: None,
            host: None,
            include_codec_tag: None,
            port: None,
            ssl: None,
            ssl_certificate: None,
            ssl_certificate_authorities: None,
            ssl_handshake_timeout: None,
            ssl_key: None,
            ssl_key_passphrase: None,
            ssl_verify_mode: None,
            ssl_peer_metadata: None,
            tls_max_version: None,
            tls_min_version: None,
        }
    }
}
