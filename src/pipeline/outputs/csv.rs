#![allow(unused)]

// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-csv.html

use super::CommonOptions;
use std::collections::HashMap;
use std::path::Path;

pub struct CsvOutput {
    create_if_deleted: Option<bool>,
    csv_options: Option<HashMap<&'static str, &'static str>>,
    dir_mode: Option<i32>,
    fields: Vec<&'static str>,
    file_mode: Option<i32>,
    filename_failure: Option<&'static str>,
    flush_interval: Option<u64>,
    gzip: Option<bool>,
    path: Option<&'static Path>,
    spreadsheet_safe: Option<bool>,
    _common: CommonOptions<'static>,
}

impl CsvOutput {
    fn new(fields: Vec<&'static str>) -> Self {
        Self {
            create_if_deleted: Some(true),
            csv_options: None,
            dir_mode: Some(-1),
            fields,
            file_mode: Some(-1),
            filename_failure: Some("_filepath_failures"),
            flush_interval: Some(2),
            gzip: Some(false),
            path: None,
            spreadsheet_safe: Some(true),
            _common: CommonOptions::default(),
        }
    }
}
