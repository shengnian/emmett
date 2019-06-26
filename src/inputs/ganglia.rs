// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-ganglia.html

/// Read ganglia packets from the network via UDP.
pub struct Ganglia {
    /// The address to listen on.
    pub host: String,
    /// The port to listen on. Remember that ports less than 1024 (privileged ports) may require root to use.
    pub port: u64,
}
