#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-file.html
use std::path::Path;

#[derive(Debug)]
pub struct FileInput {
    close_older: Option<String>,
    delimiter: Option<String>,
    discover_interval: Option<u64>,
    exclude: Option<Vec<String>>,
    file_chunk_count: Option<u64>,
    file_chunk_size: Option<u64>,
    file_completed_action: Option<String>,
    file_completed_log_path: Option<String>,
    file_sort_by: Option<String>,
    file_sort_direction: Option<String>,
    ignore_older: Option<u64>,
    max_open_files: Option<u64>,
    mode: Option<String>,
    path: Option<Vec<&'static Path>>,
    sincedb_clean_after: Option<String>,
    sincedb_path: Option<String>,
    sincedb_write_interval: Option<String>,
    start_position: Option<String>,
    start_interval: Option<String>,
}

impl FileInput {
    fn new() -> Self {
        Self {
            close_older: Some("1 hour".to_string()),
            delimiter: Some("\n".to_string()),
            discover_interval: Some(15),
            exclude: None,
            file_chunk_count: Some(4_611_686_018_427_387_903),
            file_chunk_size: Some(32768),
            file_completed_action: Some("delete".to_string()),
            file_completed_log_path: None,
            file_sort_by: Some("last_modified".to_string()),
            file_sort_direction: Some("asc".to_string()),
            ignore_older: None,
            max_open_files: Some(4095),
            mode: Some("tail".to_string()),
            path: None,
            sincedb_clean_after: Some("2 weeks".to_string()),
            sincedb_path: Some("<path.data>/plugins/inputs/file".to_string()),
            sincedb_write_interval: Some("15 seconds".to_string()),
            start_position: Some("end".to_string()),
            start_interval: Some("1 second".to_string()),
        }
    }
}
