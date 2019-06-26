// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-imap.html

/// Read mails from IMAP server.

/// Periodically scan an IMAP folder (INBOX by default) and move any read messages to the trash.
pub struct Imap {
    pub check_interval: Option<u64>,
    pub content_type: Option<String>,
    pub delete: Option<bool>,
    pub expunge: Option<bool>,
    pub fetch_count: Option<u64>,
    pub folder: Option<String>,
    pub host: String,
    pub lowercase_headers: Option<bool>,
    pub password: Option<String>,
    pub port: Option<u64>,
    pub secure: Option<bool>,
    pub strip_attachment: Option<bool>,
    pub user: String,
    pub verify_cert: Option<bool>
}
