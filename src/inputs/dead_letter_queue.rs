#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-dead_letter_queue.html
use std::path::Path;

#[derive(Debug)]
pub struct DeadLetterQueueInput<'a> {
    commit_offsets: Option<bool>,
    path: &'a Path,
    pipeline_id: Option<&'a str>,
    sincedb_path: Option<&'a Path>,
    start_timestamp: Option<&'a str>,
}

impl<'a> DeadLetterQueueInput<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            commit_offsets: Some(true),
            path,
            pipeline_id: Some("main"),
            sincedb_path: Some(Path::new("/plugins/inputs/dead_letter_queue")),
            start_timestamp: None,
        }
    }
}
