#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-elasticsearch.html

use std::path::Path;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct ElasticsearchInput<'a> {
    ca_file: Option<&'a Path>,
    docinfo: Option<bool>,
    docinfo_fields: Option<Vec<&'a str>>,
    docinfo_target: Option<&'a str>,
    hosts: Vec<&'a str>,
    index: Option<&'a str>,
    password: Option<&'a str>,
    query: Option<Value>,
    schedule: Option<&'a str>,
    scroll: Option<&'a str>,
    size: Option<u64>,
    slices: Option<u64>,
    ssl: Option<bool>,
    user: Option<&'a str>
}

impl<'a> ElasticsearchInput<'a> {
    pub fn new(path: &'a Path, hosts: Vec<&'a str>) -> Self {
        Self {
            ca_file: None,
            docinfo: Some(false),
            docinfo_fields: Some(vec!["_index", "_type", "_id"]),
            docinfo_target: Some("@metadata"),
            hosts,
            index: Some("logstash-*"),
            password: None,
            query: Some(json!({ "sort": [ "_doc" ] })),
            schedule: None,
            scroll: Some("1m"),
            size: Some(1000),
            slices: None,
            ssl: Some(false),
            user: None
        }
    }
}
