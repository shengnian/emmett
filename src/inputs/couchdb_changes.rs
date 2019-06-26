// Secification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-couchdb_changes.html
use std::path::Path;

///This CouchDB input allows you to automatically stream events from the CouchDB _changes URI. Moreover, any "future" changes will automatically be streamed as well making it easy to synchronize your CouchDB data with any target destination
/// # Upsert and delete
/// You can use event metadata to allow for document deletion. All non-delete operations are treated as upserts
/// # Starting at a Specific Sequence
/// The CouchDB input stores the last sequence number value in location defined by sequence_path. You can use this fact to start or resume the stream at a particular sequence.
pub struct CouchDbChanges<'a> {
    /// Reconnect flag. When true, always try to reconnect after a failure.
    pub always_reconnect: Option<bool>,
    /// Path to a CA certificate file, used to validate certificates.
    pub ca_file: Option<&'a Path>,
    /// The CouchDB db to connect to. Required parameter.
    pub db: String,
    /// Logstash connects to CouchDB’s _changes with feed=continuous The heartbeat is how often (in milliseconds) Logstash will ping CouchDB to ensure the connection is maintained. Changing this setting is not recommended unless you know what you are doing.
    pub heartbeat: Option<u64>,
    /// IP or hostname of your CouchDB instance.
    pub host: Option<String>,
    /// Future feature! Until implemented, changing this from the default will not do anything.
    /// Ignore attachments associated with CouchDB documents.
    pub ignore_attachments: Option<u64>,
    /// If unspecified, Logstash will attempt to read the last sequence number from the sequence_path file. If that is empty or non-existent, it will begin with 0 (the beginning).
    ///If you specify this value, it is anticipated that you will only be doing so for an initial read under special circumstances and that you will unset this value afterwards.
    pub initial_sequence: Option<u64>,
    /// Preserve the CouchDB document id "_id" value in the output.
    pub keep_id: Option<bool>,
    /// Preserve the CouchDB document revision "_rev" value in the output.
    pub keep_revision: Option<bool>,
    /// Password, if authentication is needed to connect to CouchDB
    pub password: Option<String>,
    /// Port of your CouchDB instance.
    pub port: Option<u64>,
    /// Reconnect delay: time between reconnect attempts, in seconds.
    pub reconnect_delay: Option<u64>,
    /// Connect to CouchDB’s _changes feed securely (via https) Default: false (via http)
    pub secure: Option<bool>,
    /// File path where the last sequence number in the _changes stream is stored. If unset it will write to $HOME/.couchdb_seq
    pub sequence_path: Option<String>,
    /// Timeout: Number of milliseconds to wait for new data before terminating the connection. If a timeout is set it will disable the heartbeat configuration option.
    pub timeout: Option<u64>,
    /// Username, if authentication is needed to connect to CouchDB
    pub username: Option<String>,
}
