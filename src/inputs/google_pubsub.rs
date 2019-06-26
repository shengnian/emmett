// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-google_pubsub.html

use std::path::Path;

/// The main motivation behind the development of this plugin was to ingest Stackdriver Logging messages via the Exported Logs feature of Stackdriver Logging.
pub struct GooglePubsub {
    pub json_key_file: Option<&'static Path>,
    pub max_messages: u64,
    pub project_id: String,
    pub subscription: String,
    pub topic: String,
    pub include_metadata: Option<bool>,
    pub create_subscription: Option<bool>,
}
