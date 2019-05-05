#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-file.html
use std::path::Path;

#[derive(Debug)]
pub struct FileInput<'a> {
    close_older: Option<&'a str>,
    delimiter: Option<&'a str>,
    discover_interval: Option<u64>,
    exclude: Option<Vec<&'a str>>,
    file_chunk_count: Option<u64>,
    file_chunk_size: Option<u64>,
    file_completed_action: Option<&'a str>,
    file_completed_log_path: Option<&'a str>,
    file_sort_by: Option<&'a str>,
    file_sort_direction: Option<&'a str>,
    ignore_older: Option<u64>,
    max_open_files: Option<u64>,
    mode: Option<&'a str>,
    path: Vec<&'a Path>,
    sincedb_clean_after: Option<&'a str>,
    sincedb_path: Option<&'a str>,
    sincedb_write_interval: Option<&'a str>,
    start_position: Option<&'a str>,
    start_interval: Option<&'a str>,
}

impl<'a> FileInput<'a> {
    fn new(path: Vec<&'a Path>) -> Self {
        Self {
            close_older: Some("1 hour"),
            delimiter: Some("\n"),
            discover_interval: Some(15),
            exclude: None,
            file_chunk_count: Some(4611686018427387903),
            file_chunk_size: Some(32768),
            file_completed_action: Some("delete"),
            file_completed_log_path: None,
            file_sort_by: Some("last_modified"),
            file_sort_direction: Some("asc"),
            ignore_older: None,
            max_open_files: Some(4095),
            mode: Some("tail"),
            path,
            sincedb_clean_after: Some("2 weeks"),
            sincedb_path: Some("<path.data>/plugins/inputs/file"),
            sincedb_write_interval: Some("15 seconds"),
            start_position: Some("end"),
            start_interval: Some("1 second"),
        }
    }
}
