/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-beats.html

#[derive(Debug)]
pub struct BeatsInput {
    add_hostname: Option<bool>,
    cipher_suites: Option<Vec>,
    client_inactivity_timeout: Option<u64>,
    host: Option<String>,
    include_codec_tag: Option<String>,
    port: Option<String>,
    ssl: Option<bool>,
    ssl_certificate: Option<String>,
    ssl_certificate_authorities: Option<u64>,
    ssl_handshake_timeout: Option<u64>,
    ssl_key: Option<String>,
    ssk_key_passphrase: Option<String>,
    ssl_verify_mode: Option<u64>,
    ssl_peer_metadata: Option<u64>,
    tls_max_version: Option<u64>,
    tls_min_version: Option<u64>
}

impl Default for AzureEventHubsInput {

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
            tls_min_version: None
        }
    }
    
}

impl AzureEventHubsInput {

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
}
