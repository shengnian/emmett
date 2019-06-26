// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http.html

use std::collections::HashMap;
use std::path::Path;

/// Using this input you can receive single or multiline events over http(s). Applications can send an HTTP request to the endpoint started by this input and Logstash will convert it into an event for subsequent processing. Users can pass plain text, JSON, or any formatted data and use a corresponding codec with this input. For Content-Type application/json the json codec is used, but for all other data formats, plain codec is used.
///
/// This input can also be used to receive webhook requests to integrate with other services and applications. By taking advantage of the vast plugin ecosystem available in Logstash you can trigger actionable events right from your application.
pub struct Http {
    pub additional_codecs: Option<HashMap<String, String>>,
    pub cipher_suites: Option<Vec<String>>,
    pub host: Option<String>,
    pub keystore: Option<&'static Path>,
    pub keystore_password: Option<String>,
    pub password: Option<String>,
    pub port: Option<u64>,
    pub max_pending_requests: Option<u64>,
    pub response_headers: Option<HashMap<String, String>>,
    pub ssl: Option<bool>,
    pub ssl_certificate: Option<&'static Path>,
    pub ssl_certificate_authorities: Option<Vec<String>>,
    pub ssl_handshake_timetout: Option<u64>,
    pub ssl_key: Option<&'static Path>,
    pub ssl_key_passphrase: Option<String>,
    pub ssl_verify_mode: Option<String>,
    pub threads: Option<u64>,
    pub tls_max_version: Option<u64>,
    pub tls_min_version: Option<u64>,
    pub user: Option<String>,
    pub verify_mode: Option<String>,
}
