// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-heartbeat.html

/// Generate heartbeat messages.
///The general intention of this is to test the performance and availability of Logstash.
pub struct Heartbeat {
    pub count: Option<u64>,
    pub interval: Option<u64>,
    pub message: Option<String>,
    pub threads: Option<u64>,
}
