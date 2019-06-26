// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-dead_letter_queue.html

use std::path::Path;

#[derive(Debug)]
/// Logstash input to read events from Logstash’s dead letter queue.
pub struct DeadLetterQueue {
    /// Specifies whether this input should commit offsets as it processes the events. Typically you specify false when you want to iterate multiple times over the events in the dead letter queue, but don’t want to save state. This is when you are exploring the events in the dead letter queue.
    pub commit_offsets: Option<bool>,

    /// Path to the dead letter queue directory that was created by a Logstash instance. This is the path from which "dead" events are read and is typically configured in the original Logstash instance with the setting path.dead_letter_queue.
    pub path: Option<&'static Path>,

    /// ID of the pipeline whose events you want to read from.
    pub pipeline_id: Option<String>,

    /// Path of the sincedb database file (keeps track of the current position of dead letter queue) that will be written to disk. The default will write sincedb files to <path.data>/plugins/inputs/dead_letter_queue.
    pub sincedb_path: Option<&'static Path>,

    /// Timestamp in ISO8601 format from when you want to start processing the events from. For example, 2017-04-04T23:40:37.
    pub start_timestamp: Option<String>,
}

impl Default for DeadLetterQueue {
    fn default() -> Self {
        Self {
            commit_offsets: Some(true),
            path: None,
            pipeline_id: Some("main".to_string()),
            sincedb_path: Some(Path::new("/plugins/inputs/dead_letter_queue")),
            start_timestamp: None,
        }
    }
}
