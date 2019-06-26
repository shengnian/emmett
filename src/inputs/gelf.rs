// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-gelf.html

/// This input will read GELF messages as events over the network, making it a good choice if you already use Graylog2 today.
pub struct Gelf {
    /// The IP address or hostname to listen on.
    pub host: Option<String>,

    /// Whether to listen for GELF messages sent over UDP.
    pub use_udp: Option<bool>,

    /// Whether to listen for GELF messages sent over TCP.
    pub use_tcp: Option<bool>,

    /// The port to listen on. Remember that ports less than 1024 (privileged ports) may require root to use. port_tcp and port_udp can be used to set a specific port for each protocol.
    pub port: Option<u64>,

    /// Tcp port to listen on. Use port instead of this setting unless you need a different port for udp than TCP.
    pub port_tcp: Option<u64>,

    /// Udp port to listen on. Use port instead of this setting unless you need a different port for udp than tcp.
    pub port_udp: Option<u64>,

    /// Whether or not to remap the GELF message fields to Logstash event fields or leave them intact.
    /// Remapping converts the following GELF fields to Logstash equivalents:
    /// full\_message becomes event.get("message").
    /// if there is no full\_message, short\_message becomes event.get("message")
    pub remap: Option<bool>,

    /// Whether or not to remove the leading \_ in GELF fields or leave them in place. (Logstash < 1.2 did not remove them by default.). Note that GELF version 1.1 format now requires all non-standard fields to be added as an "additional" field, beginning with an underscore.
    /// e.g. \_foo becomes foo
    pub strip_leading_underscore: Option<bool>,
}
