// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-elasticsearch.html
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug)]
/// Read from an Elasticsearch cluster, based on search query results. This is useful for replaying test logs, reindexing, etc. You can periodically schedule ingestion using a cron syntax (see schedule setting) or run the query one time to load data into Logstash.
pub struct Elasticsearch<'a> {
    pub ca_file: Option<&'a Path>,
    pub docinfo: Option<bool>,
    pub docinfo_fields: Option<Vec<String>>,
    pub docinfo_target: Option<String>,
    pub hosts: Vec<String>,
    pub index: Option<String>,
    pub password: Option<String>,
    pub query: Option<Value>,
    pub schedule: Option<String>,
    pub scroll: Option<String>,
    pub size: Option<u64>,
    pub slices: Option<u64>,
    pub ssl: Option<bool>,
    pub user: Option<String>,
}

impl<'a> Elasticsearch<'a> {
    pub fn new(path: &'a Path, hosts: Vec<String>) -> Self {
        Self {
            ca_file: None,
            docinfo: Some(false),
            docinfo_fields: Some(vec![
                "_index".to_string(),
                "_type".to_string(),
                "_id".to_string(),
            ]),
            docinfo_target: Some("@metadata".to_string()),
            hosts,
            index: Some("logstash-*".to_string()),
            password: None,
            query: Some(json!({ "sort": [ "_doc" ] })),
            schedule: None,
            scroll: Some("1m".to_string()),
            size: Some(1000),
            slices: None,
            ssl: Some(false),
            user: None,
        }
    }
}
