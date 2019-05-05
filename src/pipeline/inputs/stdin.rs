#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-input-stdin.html

#[derive(Debug)]
pub struct Stdin {
    add_field: Option<String>,
    codec: Option<String>,
    enable_metric: Option<bool>,
    id: Option<String>,
    tags: Option<Vec<String>>,
    r#type: Option<String>,
}

impl Default for Stdin {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: None,
            enable_metric: None,
            id: None,
            tags: None,
            r#type: None,
        }
    }
}

impl Stdin {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
